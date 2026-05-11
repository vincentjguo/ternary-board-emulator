use std::fmt::Display;
use crate::components::platform::bus::Bus;
use crate::components::platform::wire::Wire;
use crate::types::Trit;
use crate::types::WORD_SIZE;

/// A Word is a fixed-size array of 9 Trits or 3 Trytes, representing a unit of data in the system.
/// 0 index is the most significant trit
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


impl<Idx> std::ops::Index<Idx> for Word
where
    Idx: std::slice::SliceIndex<[Trit]>,
{
    type Output = <Idx as std::slice::SliceIndex<[Trit]>>::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &(&self.trits[..])[index]
    }
}

impl FromIterator<Trit> for Word {
    fn from_iter<I: IntoIterator<Item=Trit>>(iter: I) -> Self {
        let trits: Vec<Trit> = iter.into_iter().collect();
        let trits: [Trit; WORD_SIZE] = trits.try_into()
            .expect("Word::from_iter expected WORD_SIZE trits");

        Word { trits }
    }
}


impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for trit in self.trits.iter() {
            write!(f, "{:?}", trit)?;
        }
        Ok(())
    }
}