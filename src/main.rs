mod components;
mod gates;
mod types;
mod tools;

use log::info;
use crate::components::addsub::AddSub;
use crate::components::{Component, UnaryBusOutputComponent, UnaryWireOutputComponent};
use crate::components::wire::read;
use crate::types::Trit;

fn main() {
    pretty_env_logger::init();
    info!("Hello, world! Trit example: {:?}", Trit::P);

    info!("input: {}", tools::convert_int_to_word(-3280));
    info!("input2: {}", tools::convert_int_to_word(0));
    let bus1 = components::bus::Bus::from_word(&tools::convert_int_to_word(-3280));
    let bus2 = components::bus::Bus::from_word(&tools::convert_int_to_word(0));
    let mut alu = AddSub::new(bus1, bus2, components::wire::wire(Trit::P));
    alu.update();
    info!("{}", alu.o_bus1().read_word());
    info!("{:?}", read(&alu.o_wire1()))
}
