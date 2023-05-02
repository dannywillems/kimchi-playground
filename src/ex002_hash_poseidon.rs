use kimchi::circuits::constraints::ConstraintSystem;
use kimchi::circuits::gate::CircuitGate;
use kimchi::circuits::wires::Wire;
use kimchi::circuits::{constraints, polynomials::generic::GenericGateSpec};

// We must have this import for the trait defining sponge_params.
use kimchi::curve::KimchiCurve;
use kimchi::mina_poseidon::poseidon::ArithmeticSpongeParams;
use kimchi::prover_index::testing::new_index_for_test;
use mina_curves::pasta::{Fp, Vesta};

// Imported from kimchi-visu

fn main() {
    // The public inputs will be the output of a permutation of Poseidon.
    // We use a Poseidon instance of state 3.
    let public = 3;
    let poseidon_params = Vesta::sponge_params();
    let (gates, row) = {
        // We keep all the gates we create in a list.
        // It will be useful to wire the columns later.
        let mut gates: Vec<CircuitGate<Fp>> = vec![];

        // Public inputs. We will create a gate for each public input.
        // This code is pretty generic.
        // The output, row, is an integer which tracks the next row in the
        // constraint system.
        let row = {
            for i in 0..public {
                // We create a gate for each public input.
                // We must create a wire for each public input.
                // The public inputs are always the first rows.
                // Therefore, we use Wire::for_row(i) for the ith public inputs.
                let gate: CircuitGate<Fp> = CircuitGate::<Fp>::create_generic_gadget(
                    Wire::for_row(i),
                    GenericGateSpec::Pub,
                    None,
                );
                gates.push(gate);
            }
            public
        };

        // Here we call the poseidon gadget.
        let row = {
            let round_constants = &poseidon_params.round_constants;
            let (gate, row) = CircuitGate::<Fp>::create_poseidon_gadget(
                row,
                [Wire::for_row(row), Wire::for_row(row + 11)],
                round_constants,
            );
            gates.extend(gate);
            row
        };

        // Now we will link the output of a permutation of Poseidon with the
        // input.
        {
            /* We have something like this
                                |--- W1 ---|--- W2 ---|--- W3 ---| ... |--- W13 ---|--- W14 ---|--- W15 ---|
                           |----|   PI1    |          |          | ... |           |           |           | G1 ----> Public input
                         -------|   PI2    |          |          | ... |           |           |           | G2 ----> Public input
                      ---|-|----|   PI3    |          |          |     |           |           |           | G3 ----> Public input
                      |  | |                                                                                  |
                      |  | |    |                       Poseidon permutation Gadget                           |
                      |  | |    |          |          |          |     |           |           |           |  |
            PI1 = O1  |  | -----|    O1    |    O2    |    O3    |     |           |           |           | Gn ----> Final state of Poseidon
                      |  |      |--------------------------------------------------------------------------|
                      |  |                       |          |
            PI2 = O2  |  |-----------------------|          |
                      |                                     |
            PI3 = O3  --------------------------------------|

                                O1, O2 and O3 must be equal to PI1, PI2 and PI3
                         */
            // The three first gates are the public inputs. We want to connect
            // them to the output of Poseidon.
            // When we initialise the public input, we always use the first
            // wire.
            // The attribute wires is a list of 7 wires (7 being the number of
            // wires playing a role in the permutation argument).
            // All the Poseidon outputs are on the same row, but in the first
            // three wires.
            gates[0].wires[0] = Wire { row, col: 0 };
            gates[1].wires[0] = Wire { row, col: 1 };
            gates[2].wires[0] = Wire { row, col: 2 };

            // Question: why row 0 and not 1 2 with same col?
            let poseidon_output = &mut gates[row].wires;
            poseidon_output[0] = Wire { row: 0, col: 0 };
            poseidon_output[1] = Wire { row: 1, col: 0 };
            poseidon_output[2] = Wire { row: 2, col: 0 };
        }
        (gates, row)
    };
    let cs = ConstraintSystem::<Fp>::create(gates)
        .public(public)
        .build()
        .unwrap();
    println!("{}");
}
