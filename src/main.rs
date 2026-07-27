mod components;
mod conversions;
pub mod emulator;
mod types;

use crate::conversions::{convert_int_to_unsigned_word, convert_int_to_word};
use crate::emulator::Emulator;
use crate::types::{Trit, Word};

fn reg_field(index: i32) -> [Trit; 2] {
    let word = convert_int_to_unsigned_word(index);
    [*word.get_trit(7), *word.get_trit(8)]
}

fn imm_field(value: i32) -> [Trit; 2] {
    let word = convert_int_to_word(value);
    [*word.get_trit(7), *word.get_trit(8)]
}

fn encode_addi(dst: i32, src: i32, imm: i32) -> Word {
    let s = reg_field(src);
    let t = reg_field(dst);
    let i = imm_field(imm);

    Word::from_trits([
        Trit::P,
        Trit::Z,
        Trit::P,
        s[0],
        s[1],
        t[0],
        t[1],
        i[0],
        i[1],
    ])
}

fn encode_add(dst: i32, src_a: i32, src_b: i32) -> Word {
    let s = reg_field(src_a);
    let t = reg_field(src_b);
    let d = reg_field(dst);

    Word::from_trits([
        Trit::P,
        Trit::Z,
        Trit::Z,
        s[0],
        s[1],
        t[0],
        t[1],
        d[0],
        d[1],
    ])
}

fn encode_empty() -> Word {
    Word::from_trits([Trit::Z; 9])
}

fn main() {
    pretty_env_logger::init();

    let mut emulator = Emulator::new();

    // r0 = 1, r1 = 1, r2 = r0 + r1, then shut down on the empty instruction.
    let program = vec![
        encode_addi(0, 0, 1),
        encode_addi(1, 1, 1),
        encode_add(2, 0, 1),
        encode_empty(),
    ];

    emulator.load_program(program);
    emulator.execute();

    println!("{}", emulator.dump_registers());
}
