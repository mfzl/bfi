use std::io::Read;

struct VM<R: Read> {
    tape: R,
    memory: Vec<u8>,
    ptr: usize,
}

impl<R: Read> VM<R> {
    fn new(reader: R) -> Self {
        VM {
            tape: reader,
            memory: vec![0],
            ptr: 0,
        }
    }

    fn eval(&mut self, buf: &[u8], counter: usize) -> usize {
        let mut c = counter;
        loop {
            if c > buf.len() - 1 {
                break;
            }

            match buf[c] as char {
                '>' => {
                    self.ptr += 1;
                    // ever growing memory
                    if self.memory.len() - 1 < self.ptr {
                        self.memory.push(0);
                    }
                }
                '<' => {
                    self.ptr -= 1;
                }
                '+' => {
                    self.memory[self.ptr] += 1;
                }
                '-' => {
                    self.memory[self.ptr] -= 1;
                }
                '.' => {
                    print!("{}", self.memory[self.ptr] as char);
                }
                '[' => {
                    c = self.eval(buf, c + 1) - 1; // `c` gets incremented so we deduct 1
                }
                ']' => {
                    if self.memory[self.ptr] > 0 {
                        c = counter;
                        continue;
                    }

                    return c + 1;
                }
                _ => {}
            }

            c += 1;
        }

        return c;
    }

    fn run(mut self) {
        let mut buf = vec![];
        self.tape.read_to_end(&mut buf);

        self.eval(&buf, 0);
    }
}

#[test]
fn run_with_string() {
    let p = "+++++[>+++[>++>++++>+++++>++++++<<<<-]<-]>>>+++++++++.>--.<-.<++.>>++++.>-----.<<--.-.>>---.<<.>--.<<+.";
    let vm = VM::new(p.as_bytes());
    vm.run();
}

fn main() {
    println!("Hello, world!");
}
