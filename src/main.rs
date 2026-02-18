use riscv_assembler::assembler::assemble;

use nomos::cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    let code = assemble("addi x1, x0, 42").unwrap().code;
    cpu.load_program(code);
    cpu.step();
    cpu.log_state();
}
