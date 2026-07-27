use crate::components::platform::bus::Bus;
use crate::components::platform::wire::{Wire, write};
use crate::types::Trit;

/// # Instruction Set Architecture
/// | instruction | opcode | encoding | description |
/// |---------|---------|---------|--------|
/// | add | +00 | register | addition |
/// | addi | +0+ | immediate | addition immediate |
/// | sub | +-0 | register | sub |
/// | subi | +-+ | immediate | sub immediate |
/// | and | 0+0 | register | and |
/// | andi | 0++ | immediate | and immediate |
/// | or | 0-0 | register | or |
/// | ori | 0-+ | immediate | or immediate |
/// | cons | 0+- | register | consensus |
/// | any | 0-- | register | any |
/// | sum | -0+ | register | sum |
/// | xor | +0- | register | nsum/xor |
/// | sll | 00- | immediate | shift left |
/// | srl | -00 | immediate | shift right |
/// | lh | ++**I** | load high | load high 5 trits most significant trit on last opcode trit |
/// | ll | +-- | load low | load low |
/// | beq | 000 | register | branch if equal |
/// | bne | 00+ | register | branch if not equal |
/// | cmp | -+0 | register | set -1 if less than, 1 if greater |
/// | cmpi | -+- | immediate | set -1 if immediate less than, 1 if greater  |
/// | jr | --0 | register | jump register |
/// | j | --+ | jump | jump |
/// | lw | -0- | immediate | load word \$t from MEM\[\$s + i\]:3 |
/// | sw | -++ | immediate | store word \$t to MEM\[\$s + i\]:3 |
/// | trap | --- | trap | trap |
///
/// This struct will take an opcode and decode it into the corresponding command and control signals
/// for the rest of the CPU components. The decoding logic will be based on the opcode encoding
/// defined in the table above.
///
/// Register encoding: `ooossttdd`
///
/// Immediate encoding: `ooossttii`
///
/// Load high encoding: `ooiiiiidd`
///
/// Load low encoding: `oooiiiidd`
///
/// Jump encoding: `oooiiiiii`
pub struct OpcodeDecoder {
    opcode: Bus,

    /// PrimaryReadRegister
    /// - `-` register `t`
    /// - `0` register `s`
    /// - `+` register `d`
    pub primary_reg: Wire,

    /// RegDst:
    /// - `-` register (t) for immediate/load commands
    /// - `+` destination register (d) for register commands
    pub reg_dst: Wire,

    /// RegWrite:
    /// - `-` write memory read to register
    /// - `+` write alu result to memory
    pub reg_write: Wire,

    /// LoadImmediate:
    /// - `-` to use load low encoding for ll instruction
    /// - `0` to not use load immediate encoding
    /// - `+` to use load high encoding for lh instruction
    pub load_immediate: Wire,

    /// ALUControl: control signals for the ALU to perform the correct operation [ALU signals][crate::components::alu::ALU]
    pub alu_control: Bus,

    /// MemControl:
    /// - `-` read from memory
    /// - `0` do nothing
    /// - `+` write to memory
    pub mem_control: Wire,

    /// Branch:
    /// - `-` to branch if not equal
    /// - `0` to not branch
    /// - `+` to branch if equal
    pub branch: Wire,

    /// ALUSrc:
    /// - `-` use second register data as second ALU operand
    /// - `+` use immediate value as second ALU operand
    pub alu_src: Wire,

    /// Jump:
    /// - `-` if the instruction is a jump encoding jump instruction
    /// - `0` if the instruction is not a jump instruction
    /// - `+` if the instruction is a register encoding jump instruction
    pub jump: Wire,

    /// Trap: `+` if the instruction is a trap instruction (for trap) // TODO: have syscalls use a different signal to differentiate?
    pub trap: Wire,
}

impl OpcodeDecoder {
    pub fn new(opcode: Bus) -> Self {
        Self {
            opcode,
            primary_reg: Wire::default(),
            reg_dst: Wire::default(),
            load_immediate: Wire::default(),
            alu_control: Bus::new(),
            mem_control: Wire::default(),
            reg_write: Wire::default(),
            branch: Wire::default(),
            alu_src: Wire::default(),
            jump: Wire::default(),
            trap: Wire::default(),
        }
    }

    fn reset_outputs(&self) {
        write(&self.reg_dst, &Trit::Z);
        write(&self.load_immediate, &Trit::Z);
        write(&self.mem_control, &Trit::Z);
        write(&self.branch, &Trit::Z);
        write(&self.alu_src, &Trit::N);
        write(&self.jump, &Trit::Z);
        write(&self.trap, &Trit::Z);

        for i in 0..5 {
            self.alu_control.write_trit(i, Trit::Z); // passthrough default
        }
    }

    /// sets instruction to register format
    /// - reg_dst = P
    /// - alu_src = N
    fn set_register_format(&self) {
        write(&self.reg_dst, &Trit::P);
        write(&self.alu_src, &Trit::N);
    }

    /// sets instruction to immediate format
    /// - reg_dst = N
    /// - alu_src = P
    fn set_immediate_format(&self) {
        write(&self.reg_dst, &Trit::N);
        write(&self.alu_src, &Trit::P);
    }

    fn set_addsub_control_sum(&self, subtract: bool) {
        self.alu_control
            .write_trit(0, if subtract { Trit::N } else { Trit::P });
        self.alu_control.write_trit(4, Trit::N);
    }

    fn set_addsub_control_mult(&self) {
        self.alu_control.write_trit(1, Trit::P);
        self.alu_control.write_trit(2, Trit::P);
        self.alu_control.write_trit(4, Trit::Z);
    }

    fn set_fblock_control(&self, select_low: Trit, select_high: Trit) {
        self.alu_control.write_trit(1, select_low);
        self.alu_control.write_trit(2, select_high);
        self.alu_control.write_trit(4, Trit::Z);
    }

    fn set_shift_control(&self, left_shift: bool) {
        self.alu_control
            .write_trit(3, if left_shift { Trit::N } else { Trit::Z });
        self.alu_control.write_trit(4, Trit::P);
    }
}

impl crate::components::Component for OpcodeDecoder {
    fn update(&mut self) {
        let opcode_value = self.opcode.read_word();
        let opcode_trits = *opcode_value.get_trits();

        self.reset_outputs();

        match (opcode_trits[0], opcode_trits[1], opcode_trits[2]) {
            // add: register format
            (Trit::P, Trit::Z, Trit::Z) => {
                self.set_register_format();
                self.set_addsub_control_sum(false);
                write(&self.reg_write, &Trit::P);
            }
            // sub: register format
            (Trit::P, Trit::N, Trit::Z) => {
                self.set_register_format();
                self.set_addsub_control_sum(true);
                write(&self.reg_write, &Trit::P);
            }
            // and: register format
            (Trit::Z, Trit::P, Trit::Z) => {
                self.set_register_format();
                self.set_fblock_control(Trit::N, Trit::N);
                write(&self.reg_write, &Trit::P);
            }
            // or: register format
            (Trit::Z, Trit::N, Trit::Z) => {
                self.set_register_format();
                self.set_fblock_control(Trit::Z, Trit::N);
                write(&self.reg_write, &Trit::P);
            }
            // cons: register format
            (Trit::Z, Trit::P, Trit::N) => {
                self.set_register_format();
                self.set_fblock_control(Trit::P, Trit::N);
                write(&self.reg_write, &Trit::P);
            }
            // any: register format
            (Trit::Z, Trit::N, Trit::N) => {
                self.set_register_format();
                self.set_fblock_control(Trit::N, Trit::Z);
                write(&self.reg_write, &Trit::P);
            }
            // sum: register format
            (Trit::N, Trit::Z, Trit::P) => {
                self.set_register_format();
                self.set_fblock_control(Trit::Z, Trit::Z);
                write(&self.reg_write, &Trit::P);
            }
            // xor: register format
            (Trit::P, Trit::Z, Trit::N) => {
                self.set_register_format();
                self.set_fblock_control(Trit::P, Trit::Z);
                write(&self.reg_write, &Trit::P);
            }
            // beq: register format
            (Trit::Z, Trit::Z, Trit::Z) => {
                self.set_register_format();
                write(&self.branch, &Trit::P);
                self.set_addsub_control_sum(true);
            }
            // bne: register format
            (Trit::Z, Trit::Z, Trit::P) => {
                self.set_register_format();
                write(&self.branch, &Trit::N);
                self.set_addsub_control_sum(true);
            }
            // cmp: register format
            (Trit::N, Trit::P, Trit::Z) => {
                self.set_register_format();
                self.set_addsub_control_sum(true);
                write(&self.reg_write, &Trit::P);
            }
            // jr: register format
            (Trit::N, Trit::N, Trit::Z) => {
                self.set_register_format();
                write(&self.jump, &Trit::P);
            }

            // addi: immediate format
            (Trit::P, Trit::Z, Trit::P) => {
                self.set_immediate_format();
                self.set_addsub_control_sum(false);
                write(&self.reg_write, &Trit::P);
            }
            // subi: immediate format
            (Trit::P, Trit::N, Trit::P) => {
                self.set_immediate_format();
                self.set_addsub_control_sum(true);
                write(&self.reg_write, &Trit::P);
            }
            // andi: immediate format
            (Trit::Z, Trit::P, Trit::P) => {
                self.set_immediate_format();
                self.set_fblock_control(Trit::N, Trit::N);
                write(&self.reg_write, &Trit::P);
            }
            // ori: immediate format
            (Trit::Z, Trit::N, Trit::P) => {
                self.set_immediate_format();
                self.set_fblock_control(Trit::Z, Trit::N);
                write(&self.reg_write, &Trit::P);
            }
            // sll: immediate format
            (Trit::Z, Trit::Z, Trit::N) => {
                self.set_immediate_format();
                self.set_shift_control(true);
                write(&self.reg_write, &Trit::P);
            }
            // srl: immediate format
            (Trit::N, Trit::Z, Trit::Z) => {
                self.set_immediate_format();
                self.set_shift_control(false);
                write(&self.reg_write, &Trit::P);
            }
            // cmpi: immediate format
            (Trit::N, Trit::P, Trit::N) => {
                self.set_immediate_format();
                self.set_addsub_control_sum(true);
                write(&self.reg_write, &Trit::P);
            }
            // lw: immediate format; add immediate offset and read to register
            (Trit::N, Trit::Z, Trit::N) => {
                self.set_immediate_format();
                write(&self.reg_write, &Trit::N);
                write(&self.mem_control, &Trit::N);
                self.set_addsub_control_sum(false);
            }
            // sw: immediate format; add immediate offset and write register to memory
            (Trit::N, Trit::P, Trit::P) => {
                self.set_immediate_format();
                write(&self.mem_control, &Trit::P);
                write(&self.alu_src, &Trit::P);
                self.set_addsub_control_sum(false);
            }

            // lh: load high format; passthrough alu and write to high 5 trits of register
            (Trit::P, Trit::P, _) => {
                write(&self.reg_write, &Trit::P);
                write(&self.primary_reg, &Trit::P);
                write(&self.load_immediate, &Trit::P);
                write(&self.alu_src, &Trit::P);
                self.set_addsub_control_mult();
            }
            // ll: load low format; passthrough alu and write to low 4 trits of register
            (Trit::P, Trit::N, Trit::N) => {
                write(&self.reg_write, &Trit::N);
                write(&self.primary_reg, &Trit::P);
                write(&self.load_immediate, &Trit::N);
                write(&self.alu_src, &Trit::P);
                self.set_addsub_control_mult();
            }

            // j: jump format
            (Trit::N, Trit::N, Trit::P) => {
                write(&self.jump, &Trit::N);
            }

            // trap: special encoding; sets trap=P and leaves the other outputs cleared.
            (Trit::N, Trit::N, Trit::N) => {
                write(&self.trap, &Trit::P);
            }
        }
    }
}
