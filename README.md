# Ternary Board Emulator

Example program

```rust
let mut emulator = Emulator::new();

// r0 = 1, r1 = 1, r2 = r0 + r1, then shut down on the empty instruction.
let program = vec![
    encode_addi(0, 0, 1),
    encode_addi(1, 1, 1),
    encode_add(2, 0, 1),
    encode_empty(),
];

emulator.load_program(program);
emulator.execute();
```

## Instruction Set Architecture
| instruction | opcode | encoding | description |
|---------|---------|---------|--------|
| add | +00 | register | addition |
| addi | +0+ | immediate | addition immediate |
| sub | +-0 | register | sub |
| subi | +-+ | immediate | sub immediate |
| and | 0+0 | register | and |
| andi | 0++ | immediate | and immediate |
| or | 0-0 | register | or |
| ori | 0-+ | immediate | or immediate |
| cons | 0+- | register | consensus |
| any | 0-- | register | any |
| sum | -0+ | register | sum |
| xor | +0- | register | nsum/xor |
| sll | 00- | immediate | shift left |
| srl | -00 | immediate | shift right |
| lh | ++**I** | load high | load high 5 trits most significant trit on last opcode trit |
| ll | +-- | load low | load low |
| beq | 000 | register | branch if equal |
| bne | 00+ | register | branch if not equal |
| cmp | -+0 | register | set -1 if less than, 1 if greater |
| cmpi | -+- | immediate | set -1 if immediate less than, 1 if greater  |
| jr | --0 | register | jump register |
| j | --+ | jump | jump |
| lw | -0- | immediate | load word \$t from MEM\[\$s + i\]:3 |
| sw | -++ | immediate | store word \$t to MEM\[\$s + i\]:3 |
| trap | --- | trap | trap |
