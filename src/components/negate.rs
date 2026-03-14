use crate::components::bus::Bus;
use crate::components::{Component, UnaryWireOutputComponent};
use crate::components::wire::{read, write, Wire};
use crate::gates::mul;

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


        write(&self.out, &mul(&a, &control));
    }
}

impl UnaryWireOutputComponent for Negate {
    fn o_wire1(&self) -> &Wire {
        &self.out
    }
}