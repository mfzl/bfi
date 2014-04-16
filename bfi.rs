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

    let file = args[1];

    //let mut reader = BufferedReader::new(File::open(&Path::new(file)));
    let mut reader : ~Reader = if file == ~"-" { 
        ~stdin() as ~Reader
    } else {
        ~File::open(&Path::new(file)) as ~Reader
    };

    let vm = BrainfuckVM::new(&mut reader); // print 24
    vm.run();
}
