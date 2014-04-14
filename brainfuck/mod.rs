#![allow(dead_code)]
extern crate collections;

use std::io;
use self::collections::treemap::TreeMap;


priv struct Memory {
    mem: ~[u8],
    memptr: int,
    codeptr: uint,
    cache: TreeMap<int, Option<int>>
}

pub struct BrainfuckVM {
    code: ~str,
}

enum Token {
    Incr,
    Decr, 
    Right,
    Left,
    Read,
    Write,
    Open,
    Close
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
            cache: TreeMap::new()
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
                print!("{}", state.mem[state.memptr] as char);
            },
            ',' => {

                let x = match io::stdin().read_byte() {
                    Ok(b) => b,
                    Err(_) =>  0 
                };

                state.mem[state.memptr] = x;

            },
            '[' => {

                state.cache.insert(state.memptr, None);

                let mut depth = 0;
                if state.mem[state.memptr] == 0 {
                    state.codeptr += 1;
                    let mut currentChar = self.code[state.codeptr] as char;
                    while depth > 0 || currentChar  != ']' {
                        if currentChar == '[' {
                            depth += 1;
                        } else if currentChar == ']' {
                            depth -= 1;
                        }
                        state.codeptr += 1;
                        currentChar = self.code[state.codeptr] as char;
                    }
                }
            },
            ']' => {
                let mut depth = 0;
                if state.mem[state.memptr] != 0 {
                    state.codeptr -= 1;
                    let mut currentChar = self.code[state.codeptr] as char;
                    while depth > 0 || currentChar != '[' {

                        if currentChar == ']' {
                            depth += 1;
                        } else if currentChar == '[' {
                            depth -= 1;
                        }
                        state.codeptr -= 1;
                        currentChar = self.code[state.codeptr] as char;
                    }
                }

            },
            _ => {
                // Ignore other characters
            }
        };


        //print!("Character: [{}] ", c);
        //print!("Memory: [{}]", state.mem);
        //print!("Memory Pointer: {} [{}]\n", state.memptr, state.mem[state.memptr]);
        state.codeptr += 1;
        return true;
    }
}
