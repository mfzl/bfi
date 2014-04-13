#![allow(dead_code)]

use std::io;

priv struct Memory {
    mem: ~[int],
    memptr: int
}

pub struct BrainfuckVM {
    code: ~str,
}


impl BrainfuckVM {
    pub fn new(code : ~str) -> BrainfuckVM {
        BrainfuckVM { 
            code: code,
        }
    }

    pub fn run(&self) {

        let mut state = Memory{
            mem: ~[0],
            memptr: 0
        };

        for instruction in self.code.chars() {

            self.eval(&mut state, instruction);
        }
    }

    fn eval(&self, state: &mut Memory, c : char) {
        match c {
            '>' => {
                state.memptr += 1;
                if state.memptr as uint == state.mem.len() {
                    state.mem.push(0);
                }
            },
            '<' => println!("Decrement Cell"),
            '+' => println!("Increment Byte"),
            '-' => println!("Decrement Byte"),
            '.' => println!("Print Byte"),
            ',' => println!("Read Byte"),
            '[' => println!("Cond Open"),
            ']' => println!("Cond Close"),
            _ => println!("Invalid")
        }
    }

    pub fn print(&self) {
        io::println(self.code);
    }
}
