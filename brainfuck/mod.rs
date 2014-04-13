#![allow(dead_code)]
use std::io;

priv struct Memory {
    mem: ~[u8],
    memptr: int,
    codeptr: int,
    depth: ~[int]

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

    pub fn run(&self) -> bool {
        let mut state = Memory{
            mem: ~[0],
            memptr: 0,
            codeptr: 0,
            depth: ~[]
        };

        while (state.codeptr as uint) < (self.code.len()) {
            if !self.eval(&mut state, self.code[state.codeptr] as char) {
                return false;
            }
        };

        return true;
    }

    fn eval(&self, state: &mut Memory, c : char) -> bool {
        match c {
            '>' => {
                state.memptr += 1;
                if state.memptr as uint > state.mem.len()-1{
                    state.mem.push(0);
                }
            },
            '<' => {

                if !(state.memptr as uint <= 0) {
                    state.memptr -= 1;
                }

            },
            '+' => {
                state.mem[state.memptr] += 1;
            },
            '-' => {
                state.mem[state.memptr] -= 1;
            },
            '.' => {
                print!("{:?}", state.mem[state.memptr] as char);
            },
            ',' => {

                let x = match io::stdin().read_byte() {
                    Ok(b) => {
                        b
                    },
                    Err(_) => {
                        println!("ERROR");
                        0
                    }
                };

                state.mem[state.memptr] = x;

            },
            '[' => {
                if state.mem[state.memptr] == 0 {
                    while self.code[state.codeptr] as char != ']' {
                        state.codeptr += 1;
                    }
                        state.codeptr += 1;
                } else {
                    state.depth.push(state.codeptr);
                }
            },
            ']' => {
                if state.mem[state.memptr] != 0 {
                    state.codeptr = match state.depth.last() {
                        None => return false,
                        Some(a) => *a
                    };
                } else {
                    state.depth.pop();
                }
            },
            _ => {
                // Ignore other characters
            }
        };
        /*
        println!("Character: {:?}", c);
        println!("Memory: {:?}", state.mem);
        println!("Depth: {:?}", state.depth);
        */
        state.codeptr += 1;
        return true;
    }
}
