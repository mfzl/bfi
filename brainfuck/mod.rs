#![allow(dead_code)]
extern crate collections;

use std::io;
use std::vec::Vec;
use std::io::Reader;
use self::collections::smallintmap::SmallIntMap;

priv struct Memory {
    mem: ~[u8],
    memptr: uint,
    codeptr: uint,
    cache: SmallIntMap<uint>
}

pub struct BrainfuckVM <'c> {
    code: ~Vec<u8>,
    length: uint
}

impl <'c> BrainfuckVM <'c> {
    pub fn new<T: Reader>(reader: &mut T) -> BrainfuckVM<'c> {

        let code = ~reader.read_to_end().ok().unwrap_or(Vec::new());
        let length = code.len().clone();

        BrainfuckVM { 
            code: code,
            length: length
        }
    }

    pub fn run(&self) -> bool {
        let mut state = Memory{
            mem: ~[0],
            memptr: 0,
            codeptr: 0,
            cache: SmallIntMap::with_capacity(self.length/4)
        };
        state.mem.grow(self.length / 2, &0);
        self.eval(&mut state)

    }


    #[inline]
    fn eval(&self, state: &mut Memory) -> bool {
        while state.codeptr < self.length {
            match *self.code.get(state.codeptr) as char {
                '>' => {
                    state.memptr += 1;
                    if state.memptr as uint > state.mem.len()-1{
                        state.mem.push(0);
                    }
                },
                '<' => if !(state.memptr as uint <= 0) {
                    state.memptr -= 1;
                },
                '+' =>  state.mem[state.memptr] += 1,
                '-' => state.mem[state.memptr] -= 1,
                '.' => print!("{}", state.mem[state.memptr] as char),
                ',' => state.mem[state.memptr] = match io::stdin().read_byte() {
                    Ok(b) => b,
                    Err(_) =>  0 
                },
                '[' =>  self.jump_to(state, ']'),
                ']' => self.jump_to(state, '['),
                _ => { }
            };
            state.codeptr += 1;
        }
        return true;
    }

    #[inline]
    fn jump_to(&self, state: &mut Memory, jumpToBrace: char) {
        let jump = match jumpToBrace {
            ']' => state.mem[state.memptr] == 0,
            '[' => state.mem[state.memptr] != 0,
            _ => false
        };
        if jump {
            self.jump_and_cache(state, jumpToBrace);
        }
    }

    #[inline]
    fn jump_and_cache(&self, state: &mut Memory, brace: char) {
        let mut depth = 0;
        let (otherBrace, incr) = if brace == '[' { (']', -1) } else { ('[', 1) };
        let start = state.codeptr;
        let found = *state.cache.find(&state.codeptr).unwrap_or(&0); 
        if found > 0 {
            state.codeptr = found;
        } else {
            state.codeptr += incr;
            let mut currentChar = *self.code.get(state.codeptr) as char;
            while depth > 0 || currentChar != brace {
                if currentChar == otherBrace {
                    depth += 1;
                } else if currentChar == brace {
                    depth -= 1;
                }
                state.codeptr += incr;
                currentChar = *self.code.get(state.codeptr) as char;
            }
            state.cache.insert(start, state.codeptr);
        }
    }
}
