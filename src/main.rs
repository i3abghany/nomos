use riscv_assembler::assembler::assemble;

use nomos::hart::Hart;
use nomos::elfutil::get_text_section;

fn main() {
    let code = get_text_section("bin").unwrap();
    let mut hart = Hart::new();
    hart.load_program(code);
    for _ in 0..10 {
        hart.step();
        hart.log_state();
    }
}
