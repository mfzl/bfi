use brainfuck::BrainfuckVM;
use std::os;
use std::io::{File, stdin};

mod brainfuck;

fn main() {

    let args = os::args();

    if args.len() < 2 {
        print!("Usage: {} filename.bf\n", args[0]);
        return;
    }

    let file = &args[1];

    let mut reader : Box<Reader> = if *file == "-" { 
        box stdin()
    } else {
        box File::open(&Path::new(file))
    };

    BrainfuckVM::new(&mut reader).run(); // print 24
}
