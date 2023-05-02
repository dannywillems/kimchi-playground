use kimchi::circuits::constraints;
use kimchi::circuits::gate::CircuitGate;
use kimchi::circuits::wires::Wire;

// We will instantiate our circuits over the base field of Pasta
use mina_curves::pasta::Fp;

// Build a simple constraint system to compute x + y.
// In this file, we will build a simple constraint system to compute z = x + y,
// where x and y are private inputs. The public inputs will be the output z.
// We will after that generate a proof and verify it.

fn main() {
    // Only one public variable, the output z;
    let public = 1;
    // How to create wires?
    let public_wire = Wire::for_row(0);
    // There are different type of gates, see
    // https://github.com/o1-labs/proof-systems/blob/master/kimchi/src/circuits/gate.rs#L84
    // We want to create a simple arithmetic gate. Kimchi codebase uses
    // `Generic` for the arithmetic gate.
    // A function create_generic is provided.
    // let g = CircuitGate::<Fp>::create_generic(public_wire,
    // How to select them as public/private?
    // How to create gates?
    let gates = CircuitGate::create_generic(wires, c);
    // Constraint systems are built with gates.
    let cs = constraints::ConstraintSystem::create(gates);
    // Create a proof
    // Verify the proof
    // Print the number of constraints + some stats
    // Play with some helpers like the serialisations
}
