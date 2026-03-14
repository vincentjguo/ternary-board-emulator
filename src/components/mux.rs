use crate::components::bus::Bus;
use crate::components::{Component, IOComponent, UnaryBusOutputComponent};
use crate::components::wire::{read, write, Wire};

type OutputFn = dyn Fn(&[Bus], usize) -> Bus;



pub struct Mux<T> {
    select: Vec<Wire>,
    inputs: Vec<Box<dyn IOComponent<T>>>,

    output: Bus
}

impl<V, T: IOComponent<V>> Mux<V, T: IOComponent<V>> {
    pub fn new(
        select: Vec<Wire>,
        inputs: Vec<T>,
        output: Bus
    ) -> Self {
        assert!(
            (3usize.pow(select.len() as u32)) >= inputs.len(),
            "select lines do not cover all inputs"
        );

        Self {
            select,
            inputs,
            output
        }
    }
}

impl<T> Component for Mux<T> {
    fn update(&mut self) {
        let selected = self.select.iter().fold(0, |acc: i32, wire| acc + read(wire).value() as i32);
        self.output.write_word(&self.inputs[selected as usize].read_word());
    }
}

impl UnaryBusOutputComponent for Mux {
    fn o_bus1(&self) -> &Bus {
        &self.output
    }
}