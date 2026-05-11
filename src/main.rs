mod components;
mod types;
mod conversions;
pub mod emulator;

use log::info;
use components::platform;
use crate::components::addsub::AddSub;
use crate::components::{Component, UnaryBusOutputComponent, UnaryWireOutputComponent};
use components::platform::wire::read;
use crate::types::Trit;

fn main() {
    pretty_env_logger::init();
    
}
struct Emulator {
    
}
