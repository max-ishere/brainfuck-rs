// > Increment the data pointer (to point to the next cell to the right).
// < Decrement the data pointer (to point to the next cell to the left).
// + Increment (increase by one) the byte at the data pointer.
// - Decrement (decrease by one) the byte at the data pointer.
// . Output the byte at the data pointer.
// , Accept one byte of input, storing its value in the byte at the data pointer.
// [ If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
// ] If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command.

#[derive(PartialEq, Eq, Debug)]
enum Command {
    ForwardCell,
    BackwardCell,
    IncrementValue,
    DecrementValue,
    PrintValue,
    InputValue,
    WhileBegin,
    WhileEnd,
}

impl Command {
    fn from_char(c: char) -> Option<Command> {
        match c {
            '>' => Some(Command::ForwardCell),
            '<' => Some(Command::BackwardCell),
            '+' => Some(Command::IncrementValue),
            '-' => Some(Command::DecrementValue),
            '.' => Some(Command::PrintValue),
            ',' => Some(Command::InputValue),
            '[' => Some(Command::WhileBegin),
            ']' => Some(Command::WhileEnd),
            _ => None,
        }
    }
}

fn parse_brainfuck(src_code: &str) -> Result<Vec<Command>, String> {
    let mut parsed: Vec<Command> = Vec::new();

    for command in src_code.chars() {
        if command == '\n' {
            continue;
        }
        match Command::from_char(command) {
            Some(command) => parsed.push(command),
            _ => return Err(String::from("Invalid character")),
        }
    }
    return Ok(parsed);
}

fn main() {
    let mut program = String::new();

    println!("Brainfuck interpreter. Input your program below.");
    loop {
        match std::io::stdin().read_line(&mut program) {
            Ok(_) => break,
            Err(e) => println!(
                "Error occured while reading your program: {}\nPlease try again.",
                e
            ),
        }
    }

    let program = match parse_brainfuck(program.as_str()) {
        Ok(ok) => ok,
        Err(e) => {
            panic!("Error while parsing source: {}", e);
        }
    };

    let mut data: Vec<u8> = Vec::new();
    data.resize(1000, 0);

    run(&program, &mut data);
    println!("\nEnd of execution.");
}

fn run(program: &[Command], data: &mut Vec<u8>) {
    let mut ptr: usize = 0;
    let mut input_buffer = String::new();

    let mut open_braces: usize = 0;
    for (i, command) in program.iter().enumerate() {
        dbg!(command);

        if open_braces != 0 {
            if *command == Command::WhileBegin {
                open_braces += 1;
            } else if *command == Command::WhileEnd {
                open_braces -= 1;
            }
            dbg!("Skipping this command");
            continue;
        }

        match command {
            Command::ForwardCell => {
                ptr += 1;
                dbg!(ptr);
            }
            Command::BackwardCell => {
                ptr = if ptr > 0 { ptr - 1 } else { ptr };
                dbg!(ptr);
            }
            Command::IncrementValue => {
                *get(data, ptr) += 1;
                dbg!(*get(data, ptr));
            }
            Command::DecrementValue => data[ptr] -= 1,
            Command::PrintValue => {
                print!("{}", if ptr < data.len() { data[ptr] } else { 0 } as char);
            }
            Command::InputValue => {
                if input_buffer.len() == 0 {
                    std::io::stdin()
                        .read_line(&mut input_buffer)
                        .expect("Error while reading stdin.");
                    input_buffer.remove(input_buffer.len() - 1);
                }
                if input_buffer.len() >= 1 {
                    if ptr >= data.len() {
                        data.resize(ptr + 1, 0);
                    }
                    data[ptr] = input_buffer.remove(0) as u8;
                } else {
                    panic!("Error while reading input.")
                }
            }
            Command::WhileBegin => {
                if *get(data, ptr) != 0 {
                    run(&program[i + 1..], data);
                }
                open_braces += 1;
                continue;
            }
            Command::WhileEnd => {
                dbg!(*get(data, ptr));

                if *get(data, ptr) != 0 {
                    run(&program, data);
                }
                return;
            }
        }
    }
}

fn get(data: &mut Vec<u8>, ptr: usize) -> &mut u8 {
    if ptr >= data.len() {
        data.resize(ptr + 1, 0);
    }
    return &mut data[ptr];
}
