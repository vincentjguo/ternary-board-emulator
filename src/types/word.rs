use std::fmt::Display;
use crate::types::Trit;
use crate::types::WORD_SIZE;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
pub struct Word {
    trits: [Trit; WORD_SIZE],
}

impl Word {
    pub fn new() -> Self {
        Word {
            trits: std::array::from_fn(|_| Default::default()),
        }
    }

    pub fn state(values: [i8; WORD_SIZE]) -> Self {
        Word {
            trits: values.map(|v| Trit::state(v)),
        }
    }

    pub fn from_trits(trits: [Trit; WORD_SIZE]) -> Self {
        Word { trits }
    }

    pub fn get_trits(&self) -> &[Trit; WORD_SIZE] {
        &self.trits
    }
    
    pub fn get_trit(&self, i: usize) -> &Trit {
        &self.trits[i]
    }

    pub fn set_trit(&mut self, i: usize, value: &Trit) {
        self.trits[i].set_state(value);
    }
}

impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for trit in self.trits.iter().rev() {
            write!(f, "{:?}", trit)?;
        }
        Ok(())
    }
}