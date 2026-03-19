use std::fmt::Debug;
use crate::components::wire::{Wire, read, wire, write};
use crate::components::{BinaryWireOutputComponent, Component};
use crate::binary_functions::{cons, sum};
use crate::types::Trit;

/// Half Adder component.
/// - in1: a
/// - in2: b
/// - c_out (out1): carry out
/// - sum_out (out2): sum output
pub struct HalfAdder {
    in1: Wire,
    in2: Wire,
    c_out: Wire,
    sum_out: Wire,
}

impl HalfAdder {
    pub fn new(in1: Wire, in2: Wire) -> Self {
        HalfAdder {
            in1,
            in2,
            sum_out: wire(Trit::default()),
            c_out: wire(Trit::default()),
        }
    }
}

impl Component for HalfAdder {
    fn update(&mut self) {
        let a = read(&self.in1);
        let b = read(&self.in2);

        let sum = sum(&a, &b);
        let c_out = cons(&a, &b);

        write(&self.sum_out, &sum);
        write(&self.c_out, &c_out);
    }
}

impl BinaryWireOutputComponent for HalfAdder {
    fn o_wire1(&self) -> &Wire {
        &self.c_out
    }

    fn o_wire2(&self) -> &Wire {
        &self.sum_out
    }
}

impl Debug for HalfAdder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = read(&self.in1);
        let b = read(&self.in2);
        let sum = read(&self.sum_out);
        let c_out = read(&self.c_out);
        write!(
            f,
            "HalfAdder {{ in1: {:?}, in2: {:?}, sum_out: {:?}, c_out: {:?} }}",
            a, b, sum, c_out
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::components::half_adder::HalfAdder;
    use crate::components::wire::{read, wire, write};
    use crate::components::{BinaryWireOutputComponent, Component};
    use crate::types::Trit;

    #[test]
    fn test_half_adder() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // 0 + 0 = 0
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::Z);
        assert_eq!(read(ha.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_half_adder_neg_one_plus_zero() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // -1 + 0 = -1
        write(&in1, &Trit::N);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::Z);
        assert_eq!(read(ha.o_wire2()), Trit::N);
    }

    #[test]
    fn test_half_adder_one_plus_zero() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // 1 + 0 = 1
        write(&in1, &Trit::P);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::Z);
        assert_eq!(read(ha.o_wire2()), Trit::P);
    }

    #[test]
    fn test_half_adder_zero_plus_neg_one() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // 0 + -1 = -1
        write(&in1, &Trit::Z);
        write(&in2, &Trit::N);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::Z);
        assert_eq!(read(ha.o_wire2()), Trit::N);
    }

    #[test]
    fn test_half_adder_zero_plus_one() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // 0 + 1 = 1
        write(&in2, &Trit::P);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::Z);
        assert_eq!(read(ha.o_wire2()), Trit::P);
    }

    #[test]
    fn test_half_adder_neg_one_plus_one() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // -1 + 1 = 0
        write(&in1, &Trit::N);
        write(&in2, &Trit::P);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::Z);
    }

    #[test]
    fn test_half_adder_one_plus_neg_one() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // 1 + -1 = 0
        write(&in1, &Trit::P);
        write(&in2, &Trit::N);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::Z);
        assert_eq!(read(ha.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_half_adder_one_plus_one() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // 1 + 1 = -1 with carry +1
        write(&in1, &Trit::P);
        write(&in2, &Trit::P);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::P);
        assert_eq!(read(ha.o_wire2()), Trit::N);
    }

    #[test]
    fn test_half_adder_neg_one_plus_neg_one() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());

        let mut ha = HalfAdder::new(in1.clone(), in2.clone());

        // -1 + -1 = 1 with carry -1
        write(&in1, &Trit::N);
        write(&in2, &Trit::N);
        ha.update();
        assert_eq!(read(ha.o_wire1()), Trit::N);
        assert_eq!(read(ha.o_wire2()), Trit::P);
    }
}
