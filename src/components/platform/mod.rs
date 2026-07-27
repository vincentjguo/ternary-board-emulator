use crate::components::platform::wire::{Wire, read};

pub mod binary_functions;
pub mod binary_gate;
pub mod bus;
pub mod decoder;
pub mod memory;
pub mod mux;
pub mod wire;

/// helper function to convert select wires to an index in unsigned
fn selected_index(select: &[Wire], bias: i32) -> usize {
    let signed = select
        .iter()
        .enumerate()
        .fold(0_i32, |acc, (_, wire)| acc * 3 + read(wire).value() as i32)
        + bias;
    signed as usize
}
