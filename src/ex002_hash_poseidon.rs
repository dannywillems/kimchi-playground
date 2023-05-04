use ark_poly::EvaluationDomain;
use kimchi::circuits::constraints::ConstraintSystem;
use kimchi::circuits::gate::CircuitGate;
use kimchi::circuits::polynomials;
use kimchi::circuits::wires::{Wire, COLUMNS};
use kimchi::circuits::{constraints, polynomials::generic::GenericGateSpec};

// We must have this import for the trait defining sponge_params.
use ark_ff::Zero;
use kimchi::curve::KimchiCurve;
use kimchi::mina_poseidon::poseidon::ArithmeticSpongeParams;
use kimchi::poly_commitment::srs::{endos, SRS};
use kimchi::precomputed_srs;
use kimchi::prover_index::testing::new_index_for_test;
use kimchi::prover_index::ProverIndex;
use mina_curves::pasta::{Fp, Fq, Pallas, Vesta};
use std::array;
use std::sync::Arc;

fn main() {
    let poseidon_state_size = 3;
    // The public inputs will be the output of a permutation of Poseidon.
    // We use a Poseidon instance of state 3.
    let public = poseidon_state_size;
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

            let poseidon_output = &mut gates[row].wires;
            poseidon_output[0] = Wire { row: 0, col: 0 };
            poseidon_output[1] = Wire { row: 1, col: 0 };
            poseidon_output[2] = Wire { row: 2, col: 0 };
        }
        (gates, row)
    };

    // witness for Poseidon permutation custom constraints
    let mut witness: [Vec<Fp>; COLUMNS] = array::from_fn(|_| vec![Fp::zero(); COLUMNS]);

    // creates a random input
    let input = [Fp::from(1u32), Fp::from(2u32), Fp::from(3u32)];

    polynomials::poseidon::generate_witness(
        poseidon_state_size,
        Vesta::sponge_params(),
        &mut witness,
        input,
    );

    // No need to add lookup
    let cs = ConstraintSystem::<Fp>::create(gates)
        .public(public)
        // .disable_gates_checks(disable_gates_checks)
        .build()
        .unwrap();

    println!("CS size: {}", cs.domain.d1.size());

    // We create a SRS
    let mut srs = SRS::<Vesta>::create(cs.domain.d1.size());
    srs.add_lagrange_basis(cs.domain.d1);
    // Boxing for parallel prover
    let srs = Arc::new(srs);

    let (endo_q, _endo_r): (Fp, Fq) = endos::<Pallas>();
    let prover_index = ProverIndex::<Vesta>::create(cs, endo_q, srs);

    let group_map = KimchiCurve::Map::setup();
    let proof = ProverProof::create_recursive::<EFqSponge, EFrSponge>(
        &group_map,
        witness,
        &self.0.runtime_tables,
        &prover,
        self.0.recursion,
        None,
    );
}
