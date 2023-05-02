use kimchi::circuits::constraints;
use kimchi::circuits::gate::CircuitGate;
use kimchi::circuits::wires::Wire;

// We will instantiate our circuits over the base field of Pasta
use mina_curves::pasta::Fp;

// Build a simple constraint system to compute x + y.
// In this file, we will build a simple constraint system to compute z = x + y,
// where x and y are private inputs. The public inputs will be the output z.
// We will after that generate a proof and verify it.

fn main() {}
