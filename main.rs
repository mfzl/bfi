use brainfuck::BrainfuckVM;

mod brainfuck;

fn main() {
    let code = ~"++++>++.<.";
    let vm = BrainfuckVM::new(code); // print 24
    vm.run();
}
