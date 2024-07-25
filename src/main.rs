use std::{
    error::Error,
    io::{stdin, BufReader, Read},
};

fn main() {
    if let Err(e) = run(Default::default(), stdin().lock()) {
        eprintln!("error: {e}");
    }
}

fn run(mut m: Machine, r: impl Read) -> Result<(), Box<dyn Error>> {
    let r = BufReader::new(r);
    for b in r.bytes() {
        match parse(b?) {
            None => (),
            Some(x) => m.apply(x),
        };
    }
    Ok(())
}

fn parse(b: u8) -> Option<Command> {
    match b {
        b'>' => Some(Command::IncrementDataPointer),
        b'<' => Some(Command::DecrementDataPointer),
        b'+' => Some(Command::IncrementByte),
        b'-' => Some(Command::DecrementByte),
        b'.' => Some(Command::OutputByte),
        b',' => Some(Command::AcceptOneByte),
        b'[' => Some(Command::JumpForward),
        b']' => Some(Command::JumpBackward),
        _ => None,
    }
}

enum Command {
    IncrementDataPointer,
    DecrementDataPointer,
    IncrementByte,
    DecrementByte,
    OutputByte,
    AcceptOneByte,
    JumpForward,
    JumpBackward,
}

struct Machine {
    cells: [u8; 30000],
    p: usize,
}

impl Default for Machine {
    fn default() -> Self {
        Self {
            cells: [0; 30000],
            p: 0,
        }
    }
}

impl Machine {
    fn apply(&mut self, c: Command) {
        match c {
            Command::IncrementDataPointer => self.p += 1,
            Command::DecrementDataPointer => self.p -= 1,
            Command::IncrementByte => self.cells[self.p] += 1,
            Command::DecrementByte => self.cells[self.p] -= 1,
            Command::OutputByte => print!("{}", self.char_at_p()),
            Command::AcceptOneByte => todo!("accept one byte of input (',')"),
            Command::JumpForward => {
                if self.cells[self.p] == 0 {
                    todo!("jump forward ('[') to ']'");
                }
            }
            Command::JumpBackward => {
                if self.cells[self.p] != 0 {
                    todo!("jump backward (']') to '['");
                }
            }
        };
    }

    fn char_at_p(&self) -> char {
        self.cells[self.p] as char
    }
}
