use crate::components::platform::wire::{read, Wire};

pub mod wire;
pub mod bus;
pub mod mux;
pub mod binary_gate;
pub mod decoder;

/// helper function to convert select wires to an index in unsigned
fn selected_index(select: &[Wire], bias: i32) -> usize {
    let signed = select .iter()
        .enumerate()
        .fold(0_i32, |acc, (i, wire)| {
            acc + read(wire).value() as i32 *3_i32.pow(i as u32)
        })
        + bias;
    signed as usize}