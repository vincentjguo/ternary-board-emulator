use std::fmt::Debug;
use crate::components::half_adder::HalfAdder;
use crate::components::wire::{read, wire, write, Wire};
use crate::components::{BinaryWireOutputComponent, Component};
use crate::binary_functions::any;
use crate::types::Trit;

/// Full Adder component.
/// - in1: a
/// - in2: b
/// - c_in: carry in
/// - c_out (wire1): carry out
/// - sum_out (wire2): sum output
pub struct Adder {
    in1: Wire,
    in2: Wire,
    c_in: Wire,
    c_out: Wire,
    sum_out: Wire,

    h_add_in: HalfAdder,
    h_add_c_in: HalfAdder,
}

impl Adder {
    pub fn new(in1: Wire, in2: Wire, c_in: Wire) -> Self {
        let h_add_in = HalfAdder::new(in1.clone(), in2.clone());

        let h_add_c_in = HalfAdder::new(h_add_in.o_wire2().clone(), c_in.clone());

        Adder {
            in1,
            in2,
            c_in,
            sum_out: h_add_c_in.o_wire2().clone(),
            c_out: wire(Trit::default()),

            h_add_in,
            h_add_c_in,
        }
    }
}

impl Component for Adder {
    fn update(&mut self) {
        self.h_add_in.update();
        self.h_add_c_in.update();

        let carry = any(&read(self.h_add_in.o_wire1()), &read(self.h_add_c_in.o_wire1()));
        write(&self.c_out, &carry);
    }
}

impl BinaryWireOutputComponent for Adder {
    fn o_wire1(&self) -> &Wire {
        &self.c_out
    }

    fn o_wire2(&self) -> &Wire {
        &self.sum_out
    }
}

impl Debug for Adder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = read(&self.in1);
        let b = read(&self.in2);
        let c_in = read(&self.c_in);
        let sum_out = read(&self.sum_out);
        let c_out = read(&self.c_out);
        write!(
            f,
            "Adder {{ in1: {:?}, in2: {:?}, c_in: {:?}, sum_out: {:?}, c_out: {:?} }}",
            a, b, c_in, sum_out, c_out
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::components::adder::Adder;
    use crate::components::wire::{read, wire};
    use crate::components::{BinaryWireOutputComponent, Component};
    use crate::types::Trit;

    #[test]
    fn test_adder_zero_plus_zero_no_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + 0 = 0
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_zero_plus_zero_with_pos_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + 0 (+ 1) = 1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_zero_plus_zero_with_neg_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + 0 (+ -1) = -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_pos_plus_zero_no_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + 0 = 1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_pos_plus_zero_with_pos_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + 0 (+ 1) = -1 with carry +1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::P);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_pos_plus_zero_with_neg_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + 0 (+ -1) = 0
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_zero_plus_pos_no_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + 1 = 1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_zero_plus_pos_with_pos_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + 1 (+ 1) = -1 with carry +1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::P);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_zero_plus_pos_with_neg_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + 1 (+ -1) = 0
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_neg_plus_zero_no_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + 0 = -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_neg_plus_zero_with_pos_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + 0 (+ 1) = 0
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_neg_plus_zero_with_neg_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::default());
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + 0 (+ -1) = 1 with carry -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::N);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_zero_plus_neg_no_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + -1 = -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_zero_plus_neg_with_pos_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + -1 (+ 1) = 0
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_zero_plus_neg_with_neg_carry() {
        let in1 = wire(Trit::default());
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 0 + -1 (+ -1) = +1 with carry -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::N);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_pos_plus_neg_no_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + -1 = 0
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_pos_plus_neg_with_pos_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + -1 (+ 1) = 1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_pos_plus_neg_with_neg_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + -1 (+ -1) = -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_neg_plus_pos_no_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + 1 = 0
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_neg_plus_pos_with_pos_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + 1 (+ 1) = 1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_neg_plus_pos_with_neg_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + 1 (+ -1) = -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_pos_plus_pos_no_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + 1 = -1 with carry +1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::P);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_pos_plus_pos_with_pos_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + 1 (+ 1) = 0 with carry +1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::P);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }

    #[test]
    fn test_adder_pos_plus_pos_with_neg_carry() {
        let in1 = wire(Trit::P);
        let in2 = wire(Trit::P);
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test 1 + 1 (+ -1) = 1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_neg_plus_neg_no_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::default());

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + -1 = 1 with carry -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::N);
        assert_eq!(read(adder.o_wire2()), Trit::P);
    }

    #[test]
    fn test_adder_neg_plus_neg_with_pos_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::P);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + -1 (+ 1) = -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::Z);
        assert_eq!(read(adder.o_wire2()), Trit::N);
    }

    #[test]
    fn test_adder_neg_plus_neg_with_neg_carry() {
        let in1 = wire(Trit::N);
        let in2 = wire(Trit::N);
        let c_in = wire(Trit::N);

        let mut adder = Adder::new(in1.clone(), in2.clone(), c_in.clone());

        // Test -1 + -1 (+ -1) = 0 with carry -1
        adder.update();
        assert_eq!(read(adder.o_wire1()), Trit::N);
        assert_eq!(read(adder.o_wire2()), Trit::Z);
    }
}
