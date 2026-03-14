pub mod adder;
mod half_adder;
pub mod wire;
pub mod addsub;
pub mod bus;
mod negate;
mod mux;
pub mod fblock;
pub mod binary_gate;
pub mod alu;
pub mod shifter_rotator;

pub trait Component {
    /// read from wires and compute outputs
    fn update(&mut self);
}

pub trait IOComponent<T> {
    fn read(&mut self) -> T;
    fn write(&mut self, value: T);
}

pub trait BinaryWireOutputComponent: Component {
    fn o_wire1(&self) -> &wire::Wire;
    fn o_wire2(&self) -> &wire::Wire;
}

pub trait UnaryWireOutputComponent: Component {
    fn o_wire1(&self) -> &wire::Wire;
}

pub trait UnaryBusOutputComponent: Component {
    fn o_bus1(&self) -> &bus::Bus;
}
