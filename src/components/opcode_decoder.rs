

/// | opcode | command | header3 |
/// |---------|---------|---------|
/// | add | +00 | content3 |
/// | addi | +0+ | content3 |
/// | sub | +-0 | content3 |
/// | subi | +-+ | content3 |
/// | and | 0+0 | content3 |
/// | andi | 0++ | content3 |
/// | or | 0-0 | content3 |
/// | ori | 0-+ | content3 |
/// | cons | 0+- | content3 |
/// | any | 0-- | content3 |
/// | sum | --- | content3 |
/// | xor | 00- | content3 |
/// | sll | 00- | content3 |
/// | srl | -00 | content3 |
pub struct OpcodeDecoder {
    opcode: u8,
}