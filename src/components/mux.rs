use std::fmt::Debug;
use crate::components::bus::Bus;
use crate::components::wire::{Wire, read, write};
use crate::components::{Component, IOComponent, UnaryBusOutputComponent};

/// Multiplexer component.
/// - select: control lines to select which input to output. 0 index least significant digit in unbalanced ternary
/// - inputs: list of input buses. The number of inputs should be less than or equal to 3^select.len()
///
/// TODO: refactor to combine trit and bus mux logic
pub struct Mux {
    select: Vec<Wire>,
    inputs: Vec<Bus>,

    output: Bus,
    bias: i32
}

impl Mux {
    pub fn new(select: Vec<Wire>, inputs: Vec<Bus>, output: Bus) -> Self {
        let max_inputs = 3usize.pow(select.len() as u32);
        assert!(
            max_inputs >= inputs.len(),
            "select lines ({}) do not cover all inputs ({})", max_inputs, inputs.len()
        );

        Self {
            select,
            inputs,
            output,
            bias: max_inputs as i32 / 2, // Bias to convert from signed to unsigned index
        }
    }
}

impl Component for Mux {
    fn update(&mut self) {
        let selected = self
            .select
            .iter()
            .enumerate()
            .fold(0, |acc: i32, (i, wire)| {
                acc + read(wire).value() as i32 * (3i32.pow(i as u32))
            }) + self.bias;
        self.output
            .write_word(&self.inputs[selected as usize].read_word());
    }
}

impl UnaryBusOutputComponent for Mux {
    fn o_bus1(&self) -> &Bus {
        &self.output
    }
}

pub struct TritMux {
    select: Vec<Wire>,
    inputs: Vec<Wire>,

    output: Wire,
    bias: i32
}

impl TritMux {
    pub fn new(select: Vec<Wire>, inputs: Vec<Wire>, output: Wire) -> Self {
        let max_inputs = 3usize.pow(select.len() as u32);
        assert!(
            max_inputs >= inputs.len(),
            "select lines ({}) do not cover all inputs ({})", max_inputs, inputs.len()
        );

        Self {
            select,
            inputs,
            output,
            bias: max_inputs as i32 / 2, // Bias to convert from signed to unsigned index
        }
    }
}

impl Component for TritMux {
    fn update(&mut self) {
        let selected = self
            .select
            .iter()
            .enumerate()
            .fold(0, |acc: i32, (i, wire)| {
                acc + read(wire).value() as i32 * (3i32.pow(i as u32))
            }) + self.bias;
        write(&self.output, &read(&self.inputs[selected as usize]));
    }
}

impl Debug for TritMux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let selected = self
            .select
            .iter()
            .enumerate()
            .fold(0, |acc: i32, (i, wire)| {
                acc + read(wire).value() as i32 * (3i32.pow(i as u32))
            }) + self.bias;
        write!(f, "TritMux {{ select: {:?}, output: {:?} }}", self.select.iter().map(|w| read(w)).collect::<Vec<_>>(), read(&self.inputs[selected as usize]))
    }
}

mod tests {
    use crate::components::mux::{Mux, TritMux};
    use crate::components::wire::{read, write};
    use crate::components::{Component, UnaryBusOutputComponent};
    use crate::tools::convert_int_to_word;
    use crate::types::Trit;

    #[test]
    fn test_mux() {
        let select = vec![
            crate::components::wire::wire(Trit::N),
            crate::components::wire::wire(Trit::N),
        ];
        let inputs = vec![
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                1, 0, 0, 0, 0, 0, 0, 0, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 1, 0, 0, 0, 0, 0, 0, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 0, 1, 0, 0, 0, 0, 0, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 0, 0, 1, 0, 0, 0, 0, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 0, 0, 0, 1, 0, 0, 0, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 0, 0, 0, 0, 1, 0, 0, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 0, 0, 0, 0, 0, 1, 0, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 0, 0, 0, 0, 0, 0, 1, 0,
            ])),
            crate::components::bus::Bus::from_word(&crate::types::Word::state([
                0, 0, 0, 0, 0, 0, 0, 0, 1,
            ])),
        ];
        let output = crate::components::bus::Bus::new();
        let mut mux = Mux::new(select.clone(), inputs.clone(), output.clone());

        // N, N ->0
        write(&select[0], &Trit::N);
        write(&select[1], &Trit::N);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[0].read_word());

        // Z, N ->1
        write(&select[0], &Trit::Z);
        write(&select[1], &Trit::N);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[1].read_word());

        // P, N ->2
        write(&select[0], &Trit::P);
        write(&select[1], &Trit::N);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[2].read_word());

        // N, Z ->3
        write(&select[0], &Trit::N);
        write(&select[1], &Trit::Z);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[3].read_word());

        // Z, Z ->4
        write(&select[0], &Trit::Z);
        write(&select[1], &Trit::Z);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[4].read_word());

        // P, Z ->5
        write(&select[0], &Trit::P);
        write(&select[1], &Trit::Z);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[5].read_word());

        // N, P ->6
        write(&select[0], &Trit::N);
        write(&select[1], &Trit::P);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[6].read_word());

        // Z, P ->7
        write(&select[0], &Trit::Z);
        write(&select[1], &Trit::P);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[7].read_word());

        // P, P ->8
        write(&select[0], &Trit::P);
        write(&select[1], &Trit::P);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[8].read_word());
    }
}
