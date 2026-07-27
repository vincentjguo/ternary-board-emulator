use crate::components::UnaryBusOutputComponent;
use crate::components::platform::binary_functions::{and, any, cons, or, sum, xor};
use crate::components::platform::binary_gate::BinaryGate;
use crate::components::platform::bus::Bus;
use crate::components::platform::mux::Mux;
use crate::components::platform::wire::Wire;
use crate::types::Trit;
use std::fmt::Debug;

/// FBlock component that can perform multiple functions based on select lines.
/// - in1: first input bus
/// - in2: second input bus
/// - out: output bus
/// - select: 2 trits to select the function (AND, OR, CONS, ANY, ADD, MUL)
pub struct FBlock {
    in1: Bus,
    in2: Bus,
    out: Bus,
    select: [Wire; 2],
    mux: Mux,
    functions: Vec<BinaryGate>,
}

const FUNCTIONS: [fn(&Trit, &Trit) -> Trit; 6] = [and, or, cons, any, sum, xor];

impl FBlock {
    pub fn new(in1: Bus, in2: Bus, select: [Wire; 2]) -> Self {
        let mut functions = Vec::new();
        let mut mux_inputs = Vec::new();

        for func in FUNCTIONS.iter() {
            let gate = BinaryGate::new(format!("{:?}", func), in1.clone(), in2.clone(), *func);
            mux_inputs.push(gate.o_bus1().clone());
            functions.push(gate);
        }

        let mux = Mux::new(select.to_vec(), mux_inputs);

        FBlock {
            in1,
            in2,
            out: mux.o_bus1().clone(),
            mux,
            select,
            functions,
        }
    }
}

impl crate::components::Component for FBlock {
    fn update(&mut self) {
        for func in self.functions.iter_mut() {
            func.update();
        }
        self.mux.update();
    }
}

impl UnaryBusOutputComponent for FBlock {
    fn o_bus1(&self) -> &Bus {
        &self.out
    }
}

impl Debug for FBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _select_values: Vec<Trit> = self
            .select
            .iter()
            .map(crate::components::wire::read)
            .collect();
        write!(
            f,
            "FBlock {{ in1: {:?}, in2: {:?}, select: {:?}, out: {:?} }}",
            self.in1.read_word(),
            self.in2.read_word(),
            self.mux,
            self.out.read_word()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::components::platform::wire::write;
    use crate::conversions::convert_int_to_unsigned_word;
    use crate::types::Word;

    #[test]
    fn test_fblock() {
        use super::*;
        use crate::components::Component;

        let in1 = Bus::new();
        let in2 = Bus::new();

        let s0 = Wire::default();
        let s1 = Wire::default();

        let mut fblock = FBlock::new(in1.clone(), in2.clone(), [s0.clone(), s1.clone()]);

        let trits = [Trit::N, Trit::Z, Trit::P];

        for (func_idx, func) in FUNCTIONS.iter().enumerate() {
            for a in trits.iter().cloned() {
                for b in trits.iter().cloned() {
                    in1.write_trit(0, a);
                    in2.write_trit(0, b);
                    fblock.update();

                    let expected: Word = Word::from_trits([
                        func(&a, &b),
                        Trit::Z,
                        Trit::Z,
                        Trit::Z,
                        Trit::Z,
                        Trit::Z,
                        Trit::Z,
                        Trit::Z,
                        Trit::Z,
                    ]);

                    let select = convert_int_to_unsigned_word(func_idx as i32);

                    write(&s0, select.get_trit(7));
                    write(&s1, select.get_trit(8));
                    fblock.update();

                    let out = fblock.o_bus1();

                    assert_eq!(
                        out.read_word(),
                        expected,
                        "Function: {:?}, Expected: {}, Got: {} for inputs a: {:?}, b: {:?}, state: {:?}",
                        func_idx,
                        expected,
                        out.read_word(),
                        a,
                        b,
                        fblock
                    );
                }
            }
        }
    }
}
