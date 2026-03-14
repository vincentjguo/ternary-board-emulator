use crate::components::bus::Bus;
use crate::components::{Component, IOComponent, UnaryBusOutputComponent};
use crate::components::wire::{read, write, Wire};


pub struct Mux {
    select: Vec<Wire>,
    inputs: Vec<Bus>,

    output: Bus
}

impl Mux {
    pub fn new(
        select: Vec<Wire>,
        inputs: Vec<Bus>,
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

impl Component for Mux {
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

#[derive(Debug)]
pub struct TritMux {
    select: Vec<Wire>,
    inputs: Vec<Wire>,

    output: Wire
}

impl TritMux {
    pub fn new(
        select: Vec<Wire>,
        inputs: Vec<Wire>,
        output: Wire
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

impl Component for TritMux {
    fn update(&mut self) {
        let selected = self.select.iter().fold(0, |acc: i32, wire| acc + read(wire).value() as i32);
        write(&self.output, &read(&self.inputs[selected as usize]));
    }
}