use crate::components::addsub::AddSub;
use crate::components::bus::Bus;
use crate::components::fblock::FBlock;
use crate::components::mux::Mux;
use crate::components::shifter::Shifter;
use crate::components::{Component, UnaryBusOutputComponent};
use std::fmt::Debug;

/// ALU (Arithmetic Logic Unit) that performs various operations based on control signals.
/// Control signals:
///
/// | Control Bits | Operation           |
/// |--------------|---------------------|
/// | +XXX-X       | Addition (bus1 + bus2) |
/// | -XXX-X       | Subtraction (bus1 - bus2) |
/// | 0XX-XX       | Pass-through (bus1) |
/// | XAB0XX       | FBlock operations selected by AB (unsigned) (AND, OR, XOR, etc.) |
/// | XXXA+X       | Shift operations in direction of A (left if A=-, right if A=0) and amount determined by B\[0:2\] (unsigned) |
pub struct ALU {
    bus1: Bus,
    bus2: Bus,
    control: Bus,

    add_sub: AddSub,
    fblock: FBlock,
    shifter: Shifter,

    mux: Mux,
}

impl ALU {
    pub fn new(bus1: Bus, bus2: Bus, control: Bus) -> Self {
        let add_sub = AddSub::new(bus1.clone(), bus2.clone(), control.get_wire(0).clone());
        let fblock = FBlock::new(
            bus1.clone(),
            bus2.clone(),
            [control.get_wire(1).clone(), control.get_wire(2).clone()],
        );
        let shifter = Shifter::new(
            bus1.clone(),
            bus2.get_wire(0).clone(),
            bus2.get_wire(1).clone(),
            control.get_wire(3).clone(),
        );
        let mux = Mux::new(
            vec![control.get_wire(4).clone()],
            vec![
                add_sub.o_bus1().clone(),
                fblock.o_bus1().clone(),
                shifter.o_bus1().clone(),
            ],
        );
        ALU {
            bus1,
            bus2,
            control,
            add_sub,
            fblock,
            shifter,
            mux,
        }
    }
}

impl Component for ALU {
    fn update(&mut self) {
        self.add_sub.update();
        self.fblock.update();
        self.shifter.update();
        self.mux.update();
    }
}

impl UnaryBusOutputComponent for ALU {
    fn o_bus1(&self) -> &Bus {
        self.mux.o_bus1()
    }
}

impl Debug for ALU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ALU")
            .field("bus1", &self.bus1)
            .field("bus2", &self.bus2)
            .field("control", &self.control)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::components::alu::ALU;
    use crate::components::bus::Bus;
    use crate::components::wire::write;
    use crate::components::{Component, IOComponent, UnaryBusOutputComponent};
    use crate::tools::{convert_int_to_unsigned_word, convert_int_to_word};
    use crate::types::Trit;

    pub fn test_alu_addsub() {
        let mut bus1 = Bus::new();
        let mut bus2 = Bus::new();
        let control = Bus::new();
        let mut alu = ALU::new(bus1.clone(), bus2.clone(), control.clone());

        write(control.get_wire(0), &Trit::P); // Set control[0] to 1 for addition
        write(control.get_wire(4), &Trit::N); // Set control[4] to - for AddSub

        bus1.write(&convert_int_to_word(5)); // bus1 = 5
        bus2.write(&convert_int_to_word(10)); // bus2 = 10

        alu.update();

        let result = alu.o_bus1().read_word();
        assert_eq!(result, convert_int_to_word(15)); // Expect 5 + 10 = 15
    }

    #[test]
    pub fn test_alu_fblock() {
        let mut bus1 = Bus::new();
        let mut bus2 = Bus::new();
        let control = Bus::new();
        let mut alu = ALU::new(bus1.clone(), bus2.clone(), control.clone());

        write(control.get_wire(1), &Trit::N); // Set control[1] to -
        write(control.get_wire(2), &Trit::N); // Set control[2] to - for AND (0 unsigned)
        write(control.get_wire(4), &Trit::Z); // Set control[4] to 0 for FBlock

        bus1.write(&convert_int_to_word(5)); // bus1 = 5 (0+--)
        bus2.write(&convert_int_to_word(3)); // bus2 = 3 (00+0)

        alu.update();

        let result = alu.o_bus1().read_word();
        assert_eq!(result, convert_int_to_word(-4), "state: {:?}", alu); // Expect 5 AND 3 = -4 (00--)
    }

    #[test]
    pub fn test_alu_shifter() {
        let mut bus1 = Bus::new();
        let mut bus2 = Bus::new();
        let control = Bus::new();
        let mut alu = ALU::new(bus1.clone(), bus2.clone(), control.clone());

        write(control.get_wire(3), &Trit::Z); // Set control[3] to 0 for right shift
        write(control.get_wire(4), &Trit::P); // Set control[4] to 1 for Shifter

        bus1.write(&convert_int_to_word(18)); // bus1 = 9 (+-00)
        bus2.write(&convert_int_to_unsigned_word(2)); // bus2 = 2 (unsigned) (0001)

        alu.update();

        let result = alu.o_bus1().read_word();
        assert_eq!(result, convert_int_to_word(2), "state: {:?}", alu); // Expect 2 (00+-)
    }
}
