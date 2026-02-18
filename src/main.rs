use riscv_assembler::assembler::{assemble, AssemblyOutput};
use riscv_decode::decode;

use nomos::fns::Exec;

fn main() {
    let mut regs: [u32; 32] = [0; 32];

    let inst = assemble("addi x1, x0, 42").unwrap().code[0];
    print!("Instruction: 0x{:08x}\n", inst);
    let decoded = decode(inst).unwrap();
    decoded.exec(&mut regs).unwrap();

    assert_eq!(regs[1], 42);
}
