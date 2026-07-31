mod components;
mod conversions;
pub mod encoders;
pub mod emulator;
mod types;

use crate::encoders::{encode_add, encode_addi, encode_empty};
use crate::emulator::Emulator;

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
