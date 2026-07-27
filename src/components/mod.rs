use platform::{bus, wire};

pub mod adder;
pub mod addsub;
pub mod alu;
pub mod detect_zero;
pub mod fblock;
mod half_adder;
pub mod immediate_extend;
mod negate;
pub mod opcode_decoder;
pub mod platform;
pub mod registers;
pub mod shifter;
pub mod trap;

pub trait Component {
    /// read from wires and compute outputs
    fn update(&mut self);
}

pub trait IOComponent<T> {
    fn read(&mut self) -> T;
    fn write(&mut self, value: &T);
}

pub trait BinaryWireOutputComponent: Component {
    fn o_wire1(&self) -> &wire::Wire;
    fn o_wire2(&self) -> &wire::Wire;
}

pub trait BinaryBusOutputComponent: Component {
    fn o_bus1(&self) -> &bus::Bus;
    fn o_bus2(&self) -> &bus::Bus;
}

pub trait UnaryWireOutputComponent: Component {
    fn o_wire1(&self) -> &wire::Wire;
}

pub trait UnaryBusOutputComponent: Component {
    fn o_bus1(&self) -> &bus::Bus;
}
