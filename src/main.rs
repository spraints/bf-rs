use std::{
    env::{args, Args},
    error::Error,
    fs::File,
    io::{stdin, BufReader, Read},
};

fn main() {
    if let Err(e) = main_impl(args()) {
        eprintln!("error: {e}");
    }
}

fn main_impl(args: Args) -> Result<(), Box<dyn Error>> {
    let (src, input) = parse_args(args)?;
    let mut program = Vec::new();
    let src = BufReader::new(src);
    for c in src.bytes() {
        if let Some(cmd) = parse(c?) {
            program.push(cmd);
        }
    }
    run(Default::default(), &program, input)
}

fn parse_args(mut args: Args) -> Result<(Box<dyn Read>, Box<dyn Read>), Box<dyn Error>> {
    let exe = args.next().ok_or("expected program name at args[0]")?;
    let program_filename = args
        .next()
        .ok_or_else(|| format!("Usage: {exe} PROGRAM <INPUT"))?;
    Ok((
        Box::new(File::open(&program_filename)?),
        Box::new(stdin().lock()),
    ))
}

fn run(mut m: Machine, program: &[Command], r: impl Read) -> Result<(), Box<dyn Error>> {
    let mut pc = 0;
    let mut input = r.bytes();
    while pc < program.len() {
        let cmd = program[pc];
        pc += 1;
        match cmd {
            Command::IncrementDataPointer => m.p += 1,
            Command::DecrementDataPointer => m.p -= 1,
            Command::IncrementByte => m.cells[m.p] += 1,
            Command::DecrementByte => m.cells[m.p] -= 1,
            Command::OutputByte => print!("{}", m.char_at_p()),
            Command::AcceptOneByte => {
                m.cells[m.p] = input.next().ok_or("[{pc}] unexpected end of input")??
            }
            Command::JumpForward => {
                if m.cells[m.p] == 0 {
                    let mut nesting = 1;
                    while nesting > 0 {
                        let cmd = program[pc];
                        pc += 1;
                        match cmd {
                            Command::JumpForward => nesting += 1,
                            Command::JumpBackward => nesting -= 1,
                            _ => (),
                        };
                    }
                }
            }
            Command::JumpBackward => {
                if m.cells[m.p] != 0 {
                    let mut nesting = 1;
                    while nesting > 0 {
                        pc -= 1;
                        match program[pc - 1] {
                            Command::JumpForward => nesting -= 1,
                            Command::JumpBackward => nesting += 1,
                            _ => (),
                        };
                    }
                }
            }
        }
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

#[derive(Clone, Copy)]
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
    fn char_at_p(&self) -> char {
        self.cells[self.p] as char
    }
}
