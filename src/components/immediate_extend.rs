use crate::components::platform::bus::Bus;
use crate::components::platform::wire::{Wire, read};
use crate::components::{Component, UnaryBusOutputComponent};
use crate::types::{Trit, Word};

/// Extends immediates depending on the encoding
pub struct ImmediateExtend {
    jump_sig: Wire,
    load_sig: Wire,

    input: Bus,
    output: Bus,
}

impl ImmediateExtend {
    pub fn new(input: Bus, jump_sig: Wire, load_sig: Wire) -> Self {
        ImmediateExtend {
            input,
            jump_sig,
            load_sig,
            output: Bus::new(),
        }
    }
}

impl Component for ImmediateExtend {
    fn update(&mut self) {
        let word = self.input.read_word();

        if read(&self.jump_sig) == Trit::P {
            let extended = Word::from_trits([
                Trit::Z,
                Trit::Z,
                Trit::Z,
                word[3],
                word[4],
                word[5],
                word[6],
                word[7],
                word[8],
            ]);
            self.output.write_word(&extended);
        } else if read(&self.jump_sig) == Trit::Z {
            let next = match read(&self.load_sig) {
                Trit::N => Word::from_trits([
                    // load low encoding, we use xor/nmult so set placeholder to N
                    Trit::N,
                    Trit::N,
                    Trit::N,
                    Trit::N,
                    Trit::N,
                    word[3],
                    word[4],
                    word[5],
                    word[6],
                ]),
                Trit::Z => Word::from_trits([
                    // immediate encoding
                    Trit::Z,
                    Trit::Z,
                    Trit::Z,
                    Trit::Z,
                    Trit::Z,
                    Trit::Z,
                    Trit::Z,
                    word[7],
                    word[8],
                ]),
                Trit::P => Word::from_trits([
                    // load high encoding
                    word[2],
                    word[3],
                    word[4],
                    word[5],
                    word[6],
                    Trit::N,
                    Trit::N,
                    Trit::N,
                    Trit::N,
                ]),
            };

            self.output.write_word(&next);
        }
    }
}

impl UnaryBusOutputComponent for ImmediateExtend {
    fn o_bus1(&self) -> &Bus {
        &self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::platform::wire::write;
    use crate::components::{Component, IOComponent};
    use crate::types::{Trit, Word};

    #[test]
    fn test_immediate_extend() {
        let jump_sig = Wire::default();
        let load_sig = Wire::default();
        let input = Bus::new();
        let mut imm_extend = ImmediateExtend::new(input, jump_sig.clone(), load_sig.clone());

        // Test jump encoding
        write(&jump_sig, &Trit::P);
        imm_extend.input.write_word(&Word::from_trits([
            Trit::P,
            Trit::P,
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::Z,
        ]));
        imm_extend.update();
        assert_eq!(
            imm_extend.output.read_word(),
            Word::from_trits([
                Trit::Z,
                Trit::Z,
                Trit::Z,
                Trit::N,
                Trit::Z,
                Trit::P,
                Trit::N,
                Trit::Z,
                Trit::Z
            ])
        );

        // Test load low encoding
        write(&jump_sig, &Trit::Z);
        write(&load_sig, &Trit::N);
        imm_extend.input.write_word(&Word::from_trits([
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::N,
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]));
        imm_extend.update();
        assert_eq!(
            imm_extend.output.read_word(),
            Word::from_trits([
                Trit::N,
                Trit::N,
                Trit::N,
                Trit::N,
                Trit::N,
                Trit::N,
                Trit::P,
                Trit::N,
                Trit::Z,
            ])
        );

        // Test load high encoding
        write(&jump_sig, &Trit::Z);
        write(&load_sig, &Trit::P);
        imm_extend.input.write_word(&Word::from_trits([
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::N,
            Trit::P,
            Trit::N,
            Trit::Z,
            Trit::Z,
            Trit::Z,
        ]));
        imm_extend.update();
        assert_eq!(
            imm_extend.output.read_word(),
            Word::from_trits([
                Trit::Z,
                Trit::N,
                Trit::P,
                Trit::N,
                Trit::Z,
                Trit::N,
                Trit::N,
                Trit::N,
                Trit::N,
            ])
        );

        // Test immediate encoding
        write(&load_sig, &Trit::Z);
        imm_extend.input.write_word(&Word::from_trits([
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::Z,
            Trit::N,
            Trit::P,
            Trit::Z,
        ]));
        imm_extend.update();
        assert_eq!(
            imm_extend.output.read_word(),
            Word::from_trits([
                Trit::Z,
                Trit::Z,
                Trit::Z,
                Trit::Z,
                Trit::Z,
                Trit::Z,
                Trit::Z,
                Trit::P,
                Trit::Z,
            ])
        );
    }
}
