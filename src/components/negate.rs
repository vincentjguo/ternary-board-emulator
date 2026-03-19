use crate::components::wire::{read, write, Wire};
use crate::components::{Component, UnaryWireOutputComponent};
use crate::binary_functions::xor;
use std::fmt::Debug;

/// Negate component.
/// - in1: input
/// - control: if 0, out = 0; if 1, out = in1; if -1, out = -in1
pub struct Negate {
    in1: Wire,
    control: Wire,
    out: Wire,
}

impl Negate {
    pub fn new(in1: Wire, control: Wire) -> Self {
        Negate { in1, control, out: Wire::new(Default::default()) }
    }
}

impl Component for Negate {
    fn update(&mut self) {
        let a = read(&self.in1);
        let control = read(&self.control);


        write(&self.out, &xor(&a, &control));
    }
}

impl UnaryWireOutputComponent for Negate {
    fn o_wire1(&self) -> &Wire {
        &self.out
    }
}

impl Debug for Negate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = read(&self.in1);
        let control = read(&self.control);
        let out = read(&self.out);
        write!(
            f,
            "Negate {{ in1: {:?}, control: {:?}, out: {:?} }}",
            a, control, out
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::components::negate::Negate;
    use crate::components::wire::{read, write};
    use crate::components::{Component, UnaryWireOutputComponent};
    use crate::types::Trit;

    #[test]
    fn test_negate_control_0() {
        let in1 = crate::components::wire::wire(Trit::N);
        let control = crate::components::wire::wire(Trit::Z);
        let mut negate = Negate::new(in1.clone(), control.clone());

        // in = -1 -> out = 0
        write(&in1, &Trit::N);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::Z);

        // in = 0 -> out = 0
        write(&in1, &Trit::Z);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::Z);

        // in = 1 -> out = 0
        write(&in1, &Trit::P);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::Z);
    }

    #[test]
    fn test_negate_control_1() {
        let in1 = crate::components::wire::wire(Trit::N);
        let control = crate::components::wire::wire(Trit::P);
        let mut negate = Negate::new(in1.clone(), control.clone());

        // in = -1 -> out = -1
        write(&in1, &Trit::N);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::N);

        // in = 0 -> out = 0
        write(&in1, &Trit::Z);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::Z);

        // in = 1 -> out = 1
        write(&in1, &Trit::P);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::P);
    }

    #[test]
    fn test_negate_control_neg1() {
        let in1 = crate::components::wire::wire(Trit::N);
        let control = crate::components::wire::wire(Trit::N);
        let mut negate = Negate::new(in1.clone(), control.clone());

        // in = -1 -> out = 1
        write(&in1, &Trit::N);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::P);

        // in = 0 -> out = 0
        write(&in1, &Trit::Z);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::Z);

        // in = 1 -> out = -1
        write(&in1, &Trit::P);
        negate.update();
        assert_eq!(read(negate.o_wire1()), Trit::N);
    }
}