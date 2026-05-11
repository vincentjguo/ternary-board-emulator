use crate::components::platform::bus::Bus;
use crate::components::platform::wire::Wire;
// TODO
pub struct Trap {
    trap_sig: Wire,
    
    pc: Bus,
    
    
    epc: Bus,
}