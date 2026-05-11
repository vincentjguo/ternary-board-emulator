use crate::components::IOComponent;
use crate::components::platform::wire::{Wire, read, wire, write};
use crate::types::{Trit, WORD_SIZE, Word};
use std::fmt::Debug;

#[derive(Clone)]
pub struct Bus([Wire; WORD_SIZE]);

impl Bus {
    pub fn new() -> Self {
        Bus(std::array::from_fn(|_| Wire::new(Default::default())))
    }

    pub fn from_word(word: &Word) -> Self {
        let trits = word.get_trits();
        let wires = std::array::from_fn(|i| wire(trits[i]));
        Bus(wires)
    }

    pub fn from_wires(wires: [Wire; WORD_SIZE]) -> Self {
        Bus(wires)
    }

    pub fn get_wire(&self, i: usize) -> &Wire {
        &self.0[i]
    }

    pub fn read_trit(&self, i: usize) -> Trit {
        read(&self.0[i])
    }

    pub fn write_trit(&self, i: usize, values: Trit) {
        write(&self.0[i], &values);
    }

    pub fn write_word(&self, word: &Word) {
        let trits = word.get_trits();
        for i in 0..WORD_SIZE {
            write(&self.0[i], &trits[i]);
        }
    }

    pub fn read_word(&self) -> Word {
        let trits = std::array::from_fn(|i| read(&self.0[i]));
        Word::from_trits(trits)
    }
}

impl Debug for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let trits = std::array::from_fn(|i| read(&self.0[i]));
        let word = Word::from_trits(trits);
        write!(f, "Bus {{{}}}", word)
    }
}

impl IOComponent<Word> for Bus {
    fn read(&mut self) -> Word {
        self.read_word()
    }

    fn write(&mut self, value: &Word) {
        self.write_word(value);
    }
}
