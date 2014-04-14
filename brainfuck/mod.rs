#![allow(dead_code)]
extern crate collections;

use std::io;
use self::collections::smallintmap::SmallIntMap;


priv struct Memory {
    mem: ~[u8],
    memptr: int,
    codeptr: uint,
    cache: SmallIntMap<uint>
}

pub struct BrainfuckVM <'c> {
    code: &'c str,
}

impl <'c> BrainfuckVM <'c> {
    pub fn new(code : &'c str) -> BrainfuckVM<'c> {
        BrainfuckVM { 
            code: code,
        }
    }

    pub fn run(&self) -> bool {
        let mut state = Memory{
            mem: ~[0],
            memptr: 0,
            codeptr: 0,
            cache: SmallIntMap::new()
        };

        if !self.eval(&mut state) {
            return false;
        }

        return true;
    }

    fn eval(&self, state: &mut Memory) -> bool {
        while (state.codeptr as uint) < (self.code.len()) {
            match self.code[state.codeptr] as char {
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


                    let start = state.codeptr;
                    let mut depth = 0;
                    if state.mem[state.memptr] == 0 {
                        let found = match state.cache.find(&state.codeptr) {
                            Some(c) => *c,
                            None => 0
                        };  
                        if found > 0 {
                            state.codeptr = found;
                        } else {
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
                            state.cache.insert(start, state.codeptr);
                        }
                    }
                },
                ']' => {
                    let start = state.codeptr;

                    let mut depth = 0;

                    if state.mem[state.memptr] != 0 {
                        let found = match state.cache.find(&state.codeptr) {
                            Some(c) => *c,
                            None => 0
                        };  

                        if found > 0 {
                            state.codeptr = found;
                        } else {
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
                            state.cache.insert(start, state.codeptr);
                        }
                    }

                },
                _ => {
                    // Ignore other characters
                }
            };

            //print!("Character: [{}] ", self.code[state.codeptr]);
            //print!("Memory: [{}]", state.mem);
            //print!("Memory Pointer: {} [{}]", state.memptr, state.mem[state.memptr]);
            //print!("Cache: {} \n", state.cache);
            state.codeptr += 1;
        }
        return true;
    }
}
