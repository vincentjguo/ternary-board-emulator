use crate::types::Trit;
use std::cmp::{max, min};
/// A binary function takes two trits as input and produces a single trit as output.
pub type BinaryFunction = fn(&Trit, &Trit) -> Trit;

// gates from https://louis-dr.github.io/ternalu3.html

/// AND gate. The output is the minimum of the two input trits.
pub fn and(a: &Trit, b: &Trit) -> Trit {
    Trit::state(min(a.value(), b.value()))
}

/// NAND gate.
pub fn nand(a: &Trit, b: &Trit) -> Trit {
    -and(a, b)
}

/// OR gate. The output is the maximum of the two input trits.
pub fn or(a: &Trit, b: &Trit) -> Trit {
    Trit::state(max(a.value(), b.value()))
}

/// NOR gate
pub fn nor(a: &Trit, b: &Trit) -> Trit {
    -or(a, b)
}

/// SUM gate
pub fn sum(a: &Trit, b: &Trit) -> Trit {
    match (a, b) {
        (Trit::N, Trit::N) => Trit::P,
        (Trit::P, Trit::P) => Trit::N,
        (a, b) => Trit::state(a.value() + b.value()),
    }
}

/// NSUM gate
pub fn nsum(a: &Trit, b: &Trit) -> Trit {
    match (a, b) {
        (Trit::N, Trit::N) => Trit::N,
        (Trit::P, Trit::P) => Trit::P,
        (a, b) => Trit::state(-(a.value() + b.value())),
    }
}

/// CONSENSUS gate
pub fn cons(a: &Trit, b: &Trit) -> Trit {
    match (a, b) {
        (Trit::N, Trit::N) => Trit::N,
        (Trit::P, Trit::P) => Trit::P,
        _ => Trit::Z,
    }
}

/// ANY gate
pub fn any(a: &Trit, b: &Trit) -> Trit {
    match (a, b) {
        (Trit::N, Trit::P) | (Trit::P, Trit::N) => Trit::Z,
        (Trit::N, _) | (_, Trit::N) => Trit::N,
        (Trit::P, _) | (_, Trit::P) => Trit::P,
        _ => Trit::Z,
    }
}

/// MULTIPLICATION gate
pub fn mult(a: &Trit, b: &Trit) -> Trit {
    Trit::state(a.value() * b.value())
}

/// XOR/NMULT gate
pub fn xor(a: &Trit, b: &Trit) -> Trit {
    Trit::state(-a.value() * b.value())
}

#[cfg(test)]
mod tests {
    use crate::components::platform::binary_functions;
    use crate::types::Trit;

    #[derive(Debug)]
    struct GateCase {
        input: Vec<Trit>,
        expected: Vec<Trit>,
    }

    fn trits(values: &[i8]) -> Vec<Trit> {
        values.iter().map(|value| Trit::state(*value)).collect()
    }

    fn case(input: &[i8], expected: &[i8]) -> GateCase {
        GateCase {
            input: trits(input),
            expected: trits(expected),
        }
    }

    fn run_cases<F>(gate_name: &str, gate: F, cases: &[GateCase])
    where
        F: Fn(&[Trit]) -> Vec<Trit>,
    {
        for (index, test_case) in cases.iter().enumerate() {
            let actual = gate(&test_case.input);
            assert_eq!(
                actual, test_case.expected,
                "{gate_name} failed at case #{index}: input={:?}",
                test_case.input
            );
        }
    }

    fn run_binary_cases(gate_name: &str, gate: fn(&Trit, &Trit) -> Trit, cases: &[GateCase]) {
        run_cases(
            gate_name,
            |input| {
                assert_eq!(
                    input.len(),
                    2,
                    "{gate_name} expects 2 input trits, got {}",
                    input.len()
                );
                vec![gate(&input[0], &input[1])]
            },
            cases,
        );
    }

    #[test]
    fn and_gate_cases() {
        let cases = vec![
            case(&[-1, -1], &[-1]),
            case(&[-1, 0], &[-1]),
            case(&[-1, 1], &[-1]),
            case(&[0, -1], &[-1]),
            case(&[0, 0], &[0]),
            case(&[0, 1], &[0]),
            case(&[1, -1], &[-1]),
            case(&[1, 0], &[0]),
            case(&[1, 1], &[1]),
        ];

        run_binary_cases("and", binary_functions::and, &cases);
    }

    #[test]
    fn sum_gate_cases() {
        let cases = vec![
            case(&[-1, -1], &[1]),
            case(&[-1, 0], &[-1]),
            case(&[-1, 1], &[0]),
            case(&[0, -1], &[-1]),
            case(&[0, 0], &[0]),
            case(&[0, 1], &[1]),
            case(&[1, -1], &[0]),
            case(&[1, 0], &[1]),
            case(&[1, 1], &[-1]),
        ];

        run_binary_cases("sum", binary_functions::sum, &cases);
    }

    #[test]
    fn nsum_gate_cases() {
        let cases = vec![
            case(&[-1, -1], &[-1]),
            case(&[-1, 0], &[1]),
            case(&[-1, 1], &[0]),
            case(&[0, -1], &[1]),
            case(&[0, 0], &[0]),
            case(&[0, 1], &[-1]),
            case(&[1, -1], &[0]),
            case(&[1, 0], &[-1]),
            case(&[1, 1], &[1]),
        ];

        run_binary_cases("nsum", binary_functions::nsum, &cases);
    }

    #[test]
    fn cons_gate_cases() {
        let cases = vec![
            case(&[-1, -1], &[-1]),
            case(&[-1, 0], &[0]),
            case(&[-1, 1], &[0]),
            case(&[0, -1], &[0]),
            case(&[0, 0], &[0]),
            case(&[0, 1], &[0]),
            case(&[1, -1], &[0]),
            case(&[1, 0], &[0]),
            case(&[1, 1], &[1]),
        ];

        run_binary_cases("cons", binary_functions::cons, &cases);
    }

    #[test]
    fn any_gate_cases() {
        let cases = vec![
            case(&[-1, -1], &[-1]),
            case(&[-1, 0], &[-1]),
            case(&[-1, 1], &[0]),
            case(&[0, -1], &[-1]),
            case(&[0, 0], &[0]),
            case(&[0, 1], &[1]),
            case(&[1, -1], &[0]),
            case(&[1, 0], &[1]),
            case(&[1, 1], &[1]),
        ];

        run_binary_cases("any", binary_functions::any, &cases);
    }

    #[test]
    fn mul_gate_cases() {
        let cases = vec![
            case(&[-1, -1], &[1]),
            case(&[-1, 0], &[0]),
            case(&[-1, 1], &[-1]),
            case(&[0, -1], &[0]),
            case(&[0, 0], &[0]),
            case(&[0, 1], &[0]),
            case(&[1, -1], &[-1]),
            case(&[1, 0], &[0]),
            case(&[1, 1], &[1]),
        ];

        run_binary_cases("mul", binary_functions::mult, &cases);
    }

    #[test]
    fn xor_gate_cases() {
        let cases = vec![
            case(&[-1, -1], &[-1]),
            case(&[-1, 0], &[0]),
            case(&[-1, 1], &[1]),
            case(&[0, -1], &[0]),
            case(&[0, 0], &[0]),
            case(&[0, 1], &[0]),
            case(&[1, -1], &[1]),
            case(&[1, 0], &[0]),
            case(&[1, 1], &[-1]),
        ];

        run_binary_cases("xor", binary_functions::xor, &cases);
    }
}
