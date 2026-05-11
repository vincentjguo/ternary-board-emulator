use crate::components::platform::bus::Bus;
use crate::components::platform::wire::{read, write, Wire};
use crate::components::{platform, BinaryWireOutputComponent, Component, UnaryBusOutputComponent, UnaryWireOutputComponent};
use std::fmt::Debug;
use crate::types::Trit;

pub(crate) trait MuxSignal: Clone {
    fn copy_to_output(src: &Self, dst: &Self);
    fn new () -> Self;
}

impl MuxSignal for Bus {
    fn copy_to_output(src: &Self, dst: &Self) {
        dst.write_word(&src.read_word());
    }
    fn new() -> Self {
        Bus::new()
    }
}

impl MuxSignal for Wire {
    fn copy_to_output(src: &Self, dst: &Self) {
        write(dst, &read(src));
    }
    fn new() -> Self {
        Wire::default()
    }
}

impl MuxSignal for [Wire; 2] {
    fn copy_to_output(src: &Self, dst: &Self) {
        for i in 0..2 {
            write(&dst[i], &read(&src[i]));
        }
    }
    fn new() -> Self {
        [Wire::default(), Wire::default()]
    }
}

/// Multiplexer component.
/// - select: control lines to select which input to output. 0 index most significant digit in unbalanced ternary
/// - inputs: list of input buses. The number of inputs should be less than or equal to 3^select.len()
pub struct SelectMux<T: MuxSignal> {
    select: Vec<Wire>,
    inputs: Vec<T>,
    output: T,
    bias: i32,
}

impl<T: MuxSignal> SelectMux<T> {
    pub fn new(select: Vec<Wire>, inputs: Vec<T>) -> Self {
        let max_inputs =3usize.pow(select.len() as u32);
        assert!(
            max_inputs >= inputs.len(),
            "select lines ({}) do not cover all inputs ({})",
            max_inputs,
            inputs.len()
        );

        Self {
            select,
            inputs,
            output: T::new(),
            bias: max_inputs as i32 /2,
        }
    }
}

impl<T: MuxSignal> Component for SelectMux<T> {
    fn update(&mut self) {
        let idx = platform::selected_index(&self.select, self.bias);
        assert!(
            idx < self.inputs.len(),
            "selected input index ({}) is not connected (inputs: {})",
            idx,
            self.inputs.len()
        );
        T::copy_to_output(&self.inputs[idx], &self.output);
    }
}

// impl<T: MuxSignal + Debug> Debug for SelectMux<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let idx = platform::selected_index(&self.select, self.bias);
//         write!(
//             f,
//             "SelectMux {{ select: {:?}, selected_index: {}, output: {:?} }}",
//             self.select.iter().map(read).collect::<Vec<_>>(),
//             idx,
//             self.output
//         )
//     }
// }

/// Specific mux for bus inputs/outputs
pub type Mux = SelectMux<Bus>;

impl UnaryBusOutputComponent for Mux {
    /// Output the selected input bus
    fn o_bus1(&self) -> &Bus {
        &self.output }
}

impl Debug for Mux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = platform::selected_index(&self.select, self.bias);
        let out = if idx < self.inputs.len() {
            self.inputs[idx].read_word()
        } else {
            // If index is not connected, show current physical output bus state.
            self.output.read_word()
        };

        write!(
            f,
            "Mux {{ select: {:?}, idx: {}, output: {:?} }}",
            self.select.iter().map(read).collect::<Vec<_>>(),
            idx,
            out )
    }
}

/// Specific mux for a single trit input/output
pub type TritMux = SelectMux<Wire>;

impl UnaryWireOutputComponent for TritMux {
    fn o_wire1(&self) -> &Wire {
        &self.output
    }
}

impl Debug for TritMux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = platform::selected_index(&self.select, self.bias);
        let out = if idx < self.inputs.len() {
            read(&self.inputs[idx])
        } else {
            // If index is not connected, show current physical output wire state.
            read(&self.output)
        };

        write!(
            f,
            "TritMux {{ select: {:?}, output: {:?} }}",
            self.select.iter().map(read).collect::<Vec<_>>(),
            out )
    }
}

pub type BinaryTritMux = SelectMux<[Wire; 2]>;

impl BinaryWireOutputComponent for BinaryTritMux {
    fn o_wire1(&self) -> &Wire {
        &self.output[0]
    }
    fn o_wire2(&self) -> &Wire {&self.output[1]}
}

impl Debug for BinaryTritMux {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = platform::selected_index(&self.select, self.bias);
        let out = [read(&self.inputs[idx][0]), read(&self.inputs[idx][1])];

        write!(
            f,
            "TritMux {{ select: {:?}, idx: {} output: {:?} }}",
            self.select.iter().map(read).collect::<Vec<_>>(),
            idx,
            out )
    }
}

/// A bus-sized mux that can be lazily initialized with inputs later
/// Used to break a dependency cycle
pub type LazyMux = Mux;

impl LazyMux {
    pub fn set_inputs(&mut self, inputs: Vec<Bus>) {
        let max_inputs =3usize.pow(self.select.len() as u32);
        assert!(
            max_inputs >= inputs.len(),
            "select lines ({}) do not cover all inputs ({})",
            max_inputs,
            inputs.len()
        );
        self.inputs = inputs;
    }
}

#[cfg(test)]
mod tests {
    use crate::components::platform::mux::Mux;
    use crate::components::platform::wire::write;
    use crate::components::{Component, UnaryBusOutputComponent};
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
        let mut mux = Mux::new(select.clone(), inputs.clone());

        // N, N ->0
        write(&select[1], &Trit::N);
        write(&select[0], &Trit::N);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[0].read_word());

        // Z, N ->1
        write(&select[1], &Trit::Z);
        write(&select[0], &Trit::N);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[1].read_word(), "{:?}", mux);

        // P, N ->2
        write(&select[1], &Trit::P);
        write(&select[0], &Trit::N);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[2].read_word());

        // N, Z ->3
        write(&select[1], &Trit::N);
        write(&select[0], &Trit::Z);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[3].read_word());

        // Z, Z ->4
        write(&select[1], &Trit::Z);
        write(&select[0], &Trit::Z);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[4].read_word());

        // P, Z ->5
        write(&select[1], &Trit::P);
        write(&select[0], &Trit::Z);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[5].read_word());

        // N, P ->6
        write(&select[1], &Trit::N);
        write(&select[0], &Trit::P);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[6].read_word());

        // Z, P ->7
        write(&select[1], &Trit::Z);
        write(&select[0], &Trit::P);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[7].read_word());

        // P, P ->8
        write(&select[1], &Trit::P);
        write(&select[0], &Trit::P);
        mux.update();
        assert_eq!(mux.o_bus1().read_word(), inputs[8].read_word());
    }
}
