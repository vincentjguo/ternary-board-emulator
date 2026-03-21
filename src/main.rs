mod components;
mod binary_functions;
mod types;
mod conversions;

use log::info;
use components::platform;
use crate::components::alu::addsub::AddSub;
use crate::components::{Component, UnaryBusOutputComponent, UnaryWireOutputComponent};
use components::platform::wire::read;
use crate::types::Trit;

fn main() {
    pretty_env_logger::init();
    info!("Hello, world! Trit example: {:?}", Trit::P);

    info!("input: {}", conversions::convert_int_to_word(-3280));
    info!("input2: {}", conversions::convert_int_to_word(0));
    let bus1 = platform::bus::Bus::from_word(&conversions::convert_int_to_word(-3280));
    let bus2 = platform::bus::Bus::from_word(&conversions::convert_int_to_word(0));
    let mut alu = AddSub::new(bus1, bus2, platform::wire::wire(Trit::P));
    alu.update();
    info!("{}", alu.o_bus1().read_word());
    info!("{:?}", read(&alu.o_wire1()))
}
