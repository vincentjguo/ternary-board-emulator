use crate::conversions::{convert_int_to_unsigned_word, convert_int_to_word, convert_word_to_int};
use crate::types::{Trit, Word};

fn reg_field(index: i32) -> [Trit; 2] {
    // Register selectors are signed ternary in this ISA: 0..4 are general-purpose registers,
    // while negative values map to the special registers.
    let word = convert_int_to_word(index);
    [*word.get_trit(7), *word.get_trit(8)]
}

fn signed_imm_field(value: i32) -> [Trit; 2] {
    let word = convert_int_to_word(value);
    [*word.get_trit(7), *word.get_trit(8)]
}

fn unsigned_imm_field(value: i32) -> [Trit; 2] {
    let word = convert_int_to_unsigned_word(value);
    [*word.get_trit(7), *word.get_trit(8)]
}

fn encode_register(opcode: [Trit; 3], dst: i32, src_a: i32, src_b: i32) -> Word {
    let s = reg_field(src_a);
    let t = reg_field(src_b);
    let d = reg_field(dst);

    Word::from_trits([opcode[0], opcode[1], opcode[2], s[0], s[1], t[0], t[1], d[0], d[1]])
}

fn encode_immediate_with(
    opcode: [Trit; 3],
    dst: i32,
    src: i32,
    imm: i32,
    imm_field: fn(i32) -> [Trit; 2],
) -> Word {
    let s = reg_field(src);
    let t = reg_field(dst);
    let i = imm_field(imm);

    Word::from_trits([opcode[0], opcode[1], opcode[2], s[0], s[1], t[0], t[1], i[0], i[1]])
}

pub fn encode_addi(dst: i32, src: i32, imm: i32) -> Word {
    encode_immediate_with([Trit::P, Trit::Z, Trit::P], dst, src, imm, signed_imm_field)
}

pub fn encode_subi(dst: i32, src: i32, imm: i32) -> Word {
    encode_immediate_with([Trit::P, Trit::N, Trit::P], dst, src, imm, signed_imm_field)
}

pub fn encode_andi(dst: i32, src: i32, imm: i32) -> Word {
    encode_immediate_with([Trit::Z, Trit::P, Trit::P], dst, src, imm, signed_imm_field)
}

pub fn encode_ori(dst: i32, src: i32, imm: i32) -> Word {
    encode_immediate_with([Trit::Z, Trit::N, Trit::P], dst, src, imm, signed_imm_field)
}

pub fn encode_sll(dst: i32, src: i32, amount: i32) -> Word {
    encode_immediate_with([Trit::Z, Trit::Z, Trit::N], dst, src, amount, unsigned_imm_field)
}

pub fn encode_srl(dst: i32, src: i32, amount: i32) -> Word {
    encode_immediate_with([Trit::N, Trit::Z, Trit::Z], dst, src, amount, unsigned_imm_field)
}

pub fn encode_cmpi(dst: i32, src: i32, imm: i32) -> Word {
    encode_immediate_with([Trit::N, Trit::P, Trit::N], dst, src, imm, signed_imm_field)
}

pub fn encode_lw(dst: i32, base: i32, offset: i32) -> Word {
    encode_immediate_with([Trit::N, Trit::Z, Trit::N], dst, base, offset, signed_imm_field)
}

pub fn encode_sw(base: i32, data: i32, offset: i32) -> Word {
    encode_immediate_with([Trit::N, Trit::P, Trit::P], data, base, offset, signed_imm_field)
}

pub fn encode_add(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::P, Trit::Z, Trit::Z], dst, src_a, src_b)
}

pub fn encode_sub(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::P, Trit::N, Trit::Z], dst, src_a, src_b)
}

pub fn encode_and(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::Z, Trit::P, Trit::Z], dst, src_a, src_b)
}

pub fn encode_or(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::Z, Trit::N, Trit::Z], dst, src_a, src_b)
}

pub fn encode_cons(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::Z, Trit::P, Trit::N], dst, src_a, src_b)
}

pub fn encode_any(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::Z, Trit::N, Trit::N], dst, src_a, src_b)
}

pub fn encode_sum(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::N, Trit::Z, Trit::P], dst, src_a, src_b)
}

pub fn encode_xor(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::P, Trit::Z, Trit::N], dst, src_a, src_b)
}

pub fn encode_empty() -> Word {
    Word::from_trits([Trit::Z; 9])
}

pub fn encode_trap() -> Word {
    Word::from_trits([Trit::N, Trit::N, Trit::N, Trit::Z, Trit::Z, Trit::Z, Trit::Z, Trit::Z, Trit::Z])
}

pub fn encode_cmp(dst: i32, src_a: i32, src_b: i32) -> Word {
    encode_register([Trit::N, Trit::P, Trit::Z], dst, src_a, src_b)
}

pub fn encode_beq(src_a: i32, src_b: i32, offset: i32) -> Word {
    encode_register([Trit::Z, Trit::Z, Trit::Z], offset, src_a, src_b)
}

pub fn encode_bne(src_a: i32, src_b: i32, offset: i32) -> Word {
    encode_register([Trit::Z, Trit::Z, Trit::P], offset, src_a, src_b)
}

pub fn encode_jr(src: i32) -> Word {
    // Register-format jump: keep the source register in the first read slot and encode a zeroed
    // destination/offset field so the instruction remains aligned with the register decoder.
    encode_register([Trit::N, Trit::N, Trit::Z], 0, src, 0)
}

pub fn encode_j(offset: i32) -> Word {
    // Jump uses the immediate/jump packing path: the lower 6 trits are the target/offset.
    let imm = convert_int_to_word(offset);
    Word::from_trits([
        Trit::N,
        Trit::N,
        Trit::P,
        *imm.get_trit(3),
        *imm.get_trit(4),
        *imm.get_trit(5),
        *imm.get_trit(6),
        *imm.get_trit(7),
        *imm.get_trit(8),
    ])
}

pub fn encode_lh(dst: i32, value: i32) -> Word {
    let imm = convert_int_to_unsigned_word(value);
    let d = reg_field(dst);
    Word::from_trits([
        Trit::P,
        Trit::P,
        *imm.get_trit(4),
        *imm.get_trit(5),
        *imm.get_trit(6),
        *imm.get_trit(7),
        *imm.get_trit(8),
        d[0],
        d[1],
    ])
}

pub fn encode_ll(dst: i32, value: i32) -> Word {
    let imm = convert_int_to_unsigned_word(value);
    let d = reg_field(dst);
    Word::from_trits([
        Trit::P,
        Trit::N,
        Trit::N,
        *imm.get_trit(5),
        *imm.get_trit(6),
        *imm.get_trit(7),
        *imm.get_trit(8),
        d[0],
        d[1],
    ])
}

#[cfg(test)]
mod tests {
    use log::info;
    use super::*;
    use crate::components::platform::binary_functions::{and, any, cons, or, sum, xor};
    use crate::emulator::Emulator;
    use crate::types::{Trit, Word};

    fn run_program(program: Vec<Word>) -> String {
        let mut emulator = Emulator::new();
        emulator.load_program(program);
        emulator.execute();
        emulator.dump_registers()
    }

    fn expected_gate_word(lhs: i32, rhs: i32, gate: fn(&Trit, &Trit) -> Trit) -> Word {
        let lhs_word = convert_int_to_word(lhs);
        let rhs_word = convert_int_to_word(rhs);

        Word::from_trits(std::array::from_fn(|i| gate(lhs_word.get_trit(i), rhs_word.get_trit(i))))
    }

    fn expected_register_line(index: usize, word: &Word) -> String {
        format!("reg{index}: {} ({})", word, convert_word_to_int(word))
    }

    fn physical_reg_index(isa_reg: i32) -> usize {
        (isa_reg + 4) as usize
    }

    fn assert_register_line(dump: &str, index: usize, expected: &Word) {
        let line = dump
            .lines()
            .find(|line| line.starts_with(&format!("reg{index}: ")))
            .unwrap_or_else(|| panic!("missing reg{index} line in dump:\n{dump}"));
        assert_eq!(line, expected_register_line(index, expected));
    }

    fn load_value_into_reg(reg: i32, value: i32) -> Vec<Word> {
        let mut program = Vec::new();
        for _ in 0..value {
            program.push(encode_addi(reg, reg, 1));
        }
        program
    }

    fn setup_program(lhs: i32, rhs: i32, operation: Word) -> Vec<Word> {
        let mut program = load_value_into_reg(0, lhs);
        program.extend(load_value_into_reg(1, rhs));
        program.push(operation);
        program.push(encode_empty());
        program
    }

    #[test]
    fn emulator_register_opcode_roundtrip() {
        pretty_env_logger::try_init().ok();

        let cases = vec![
            ("add", 1, 1, encode_add(2, 0, 1), convert_int_to_word(2)),
            ("sub", 1, 1, encode_sub(2, 0, 1), convert_int_to_word(0)),
            ("and", 1, 1, encode_and(2, 0, 1), expected_gate_word(1, 1, and)),
            ("or", 1, 1, encode_or(2, 0, 1), expected_gate_word(1, 1, or)),
            ("cons", 1, 1, encode_cons(2, 0, 1), expected_gate_word(1, 1, cons)),
            ("any", 1, 1, encode_any(2, 0, 1), expected_gate_word(1, 1, any)),
            ("sum", 1, 1, encode_sum(2, 0, 1), expected_gate_word(1, 1, sum)),
            ("xor", 1, 1, encode_xor(2, 0, 1), expected_gate_word(1, 1, xor)),
        ];

        for (name, lhs, rhs, op, expected) in cases {
            info!("Running test case: {name} with lhs={lhs}, rhs={rhs}");
            let dump = run_program(setup_program(lhs, rhs, op));
            let dump_index = physical_reg_index(2);
            assert_register_line(&dump, dump_index, &expected);
            assert!(dump.contains(&expected_register_line(dump_index, &expected)), "{name} dump:\n{dump}");
        }
    }

    #[test]
    fn emulator_shift_opcode_roundtrip() {
        pretty_env_logger::try_init().ok();

        let left_shift_dump = run_program(vec![
            encode_addi(0, 0, 1),
            encode_addi(1, 1, 1),
            encode_sll(2, 0, 1),
            encode_empty(),
        ]);
        assert_register_line(&left_shift_dump, physical_reg_index(2), &convert_int_to_word(3));

        let right_shift_dump = run_program(vec![
            encode_addi(0, 0, 1),
            encode_addi(0, 0, 1),
            encode_addi(0, 0, 1),
            encode_addi(1, 1, 1),
            encode_srl(2, 0, 1),
            encode_empty(),
        ]);
        assert_register_line(&right_shift_dump, physical_reg_index(2), &convert_int_to_word(1));
    }

    #[test]
    fn emulator_immediate_opcode_roundtrip() {
        pretty_env_logger::try_init().ok();

        let subi_dump = run_program(vec![
            encode_addi(0, 0, 1),
            encode_subi(2, 0, 1),
            encode_empty(),
        ]);
        assert_register_line(&subi_dump, physical_reg_index(2), &convert_int_to_word(0));

        let andi_dump = run_program(vec![
            encode_addi(0, 0, 1),
            encode_andi(2, 0, 1),
            encode_empty(),
        ]);
        assert_register_line(&andi_dump, physical_reg_index(2), &convert_int_to_word(1));

        let ori_dump = run_program(vec![
            encode_addi(0, 0, 1),
            encode_ori(2, 0, 0),
            encode_empty(),
        ]);
        assert_register_line(&ori_dump, physical_reg_index(2), &convert_int_to_word(1));

        let cmpi_dump = run_program(vec![
            encode_addi(0, 0, 1),
            encode_cmpi(2, 0, 1),
            encode_empty(),
        ]);
        assert_register_line(&cmpi_dump, physical_reg_index(2), &convert_int_to_word(0));
    }

    #[test]
    fn encoder_smoke_for_remaining_opcodes() {
        // These are structural smoke checks for the remaining opcode constructors.
        assert_eq!(encode_cmpi(2, 0, -1).get_trits()[0..3], [Trit::N, Trit::P, Trit::N]);
        assert_eq!(encode_lw(2, 0, -1).get_trits()[0..3], [Trit::N, Trit::Z, Trit::N]);
        assert_eq!(encode_sw(0, 1, 1).get_trits()[0..3], [Trit::N, Trit::P, Trit::P]);
        assert_eq!(encode_cmp(2, 0, 1).get_trits()[0..3], [Trit::N, Trit::P, Trit::Z]);
        assert_eq!(encode_beq(0, 1, 2).get_trits()[0..3], [Trit::Z, Trit::Z, Trit::Z]);
        assert_eq!(encode_bne(0, 1, 2).get_trits()[0..3], [Trit::Z, Trit::Z, Trit::P]);
        assert_eq!(encode_jr(0).get_trits()[0..3], [Trit::N, Trit::N, Trit::Z]);
        assert_eq!(encode_j(3).get_trits()[0..3], [Trit::N, Trit::N, Trit::P]);
        assert_eq!(encode_lh(2, 1).get_trits()[0..2], [Trit::P, Trit::P]);
        assert_eq!(encode_ll(2, 1).get_trits()[0..3], [Trit::P, Trit::N, Trit::N]);
        assert_eq!(encode_trap().get_trits()[0..3], [Trit::N, Trit::N, Trit::N]);
    }
}
