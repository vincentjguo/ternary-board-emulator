use crate::components::platform::bus::Bus;
use crate::components::platform::wire::{Wire, read};
use crate::components::{Component, UnaryBusOutputComponent};
use crate::conversions::convert_word_to_int;
use crate::types::Trit::P;
use crate::types::{Trit, WORD_SIZE, Word};
use std::fmt::Debug;

const MEMORY_SIZE: usize = 3_usize.pow(WORD_SIZE as u32);
// negative values kernel space, use bias to allow addressing the full range of memory
// TODO: check kernel mode trit on status register when accessing privileged memory addresses
pub struct Memory {
    // max addressable memory size is 3^WORD_SIZE for non-negative values
    data: [Trit; MEMORY_SIZE],

    control: Wire,

    addr_in: Bus,
    data_in: Bus,
    data_out: Bus,
}

impl Memory {
    pub fn new(control: Wire, addr_in: Bus, data_in: Bus) -> Self {
        Memory {
            data: [Trit::default(); MEMORY_SIZE],
            control,
            addr_in,
            data_in,
            data_out: Bus::new(),
        }
    }

    /// exposes the data_out bus to be modified
    /// Breaks a dependency cycle
    pub fn set_data_out_bus(&mut self, bus: Bus) {
        self.data_out = bus;
    }

    fn write(&mut self) {
        let addr = convert_word_to_int(&self.addr_in.read_word()) as usize;
        let value = self.data_in.read_word();
        for (i, trit) in value.get_trits().iter().enumerate() {
            self.data[addr + i].set_state(trit);
        }
    }

    fn read(&self) {
        let addr = convert_word_to_int(&self.addr_in.read_word()) as usize;
        let mut value = [Trit::default(); WORD_SIZE];
        value[..WORD_SIZE].copy_from_slice(&self.data[addr..(WORD_SIZE + addr)]);
        self.data_out.write_word(&Word::from_trits(value));
    }

    /// Reads the current instruction word into the output bus without consulting the memory control line.
    pub fn fetch_instruction(&mut self) {
        self.read();
    }

    // initializes data for startup
    pub fn init_data(&mut self, word: &Word, idx: usize) {
        assert!(idx < MEMORY_SIZE);
        assert_eq!(idx % WORD_SIZE, 0);
        for (i, trit) in word.get_trits().iter().enumerate() {
            self.data[idx + i].set_state(trit);
        }
    }
}

impl Component for Memory {
    fn update(&mut self) {
        match read(&self.control) {
            P => self.write(),
            Trit::N => self.read(),
            _ => {} // do nothing
        }
    }
}

impl UnaryBusOutputComponent for Memory {
    fn o_bus1(&self) -> &Bus {
        &self.data_out
    }
}

impl Debug for Memory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let addr = convert_word_to_int(&self.addr_in.read_word()) as usize;
        let value = self.data_out.read_word();
        write!(f, "Memory {{ addr: {}, value: {:?} }}", addr, value)
    }
}

#[cfg(test)]
mod tests {
    use crate::components::platform::wire::write;
    use crate::conversions::convert_int_to_word;

    #[test]
    fn test_memory() {
        use super::*;
        use crate::components::Component;

        let control = Wire::default();
        let addr_in = Bus::new();
        let data_in = Bus::new();

        let mut memory = Memory::new(control.clone(), addr_in.clone(), data_in.clone());

        // Write value 5 to address 0
        write(&control, &Trit::P);
        addr_in.write_word(&convert_int_to_word(0));
        data_in.write_word(&convert_int_to_word(5));
        memory.update();

        // Read value from address 0
        write(&control, &Trit::N);
        addr_in.write_word(&convert_int_to_word(0));
        memory.update();

        let output_value = memory.data_out.read_word();
        assert_eq!(output_value, convert_int_to_word(5));
    }
}
