use crate::components::platform::wire::{read, Wire};
use crate::types::WORD_SIZE;

pub mod wire;
pub mod bus;
pub mod mux;
pub mod binary_gate;
pub mod decoder;
pub mod memory;
pub mod binary_functions;

/// helper function to convert select wires to an index in unsigned
fn selected_index(select: &[Wire], bias: i32) -> usize {
    let signed = select .iter()
        .enumerate()
        .fold(0_i32, |acc, (_, wire)| {
            acc * 3 + read(wire).value() as i32
        })
        + bias;
    signed as usize}