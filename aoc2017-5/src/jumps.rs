use std::io::{BufRead, Cursor};

pub fn count_steps(input: &'static str) -> u32 {
    let mut instructions: Vec<i32> = read_instructions(input);
    let mut ip: i32 = 0;
    let mut steps: u32 = 0;

    loop {
        let instruction: i32 = match instructions.get(ip as usize) {
            Some(&opcode) => opcode,
            None => { break; }
        };
        instructions[ip as usize] += 1;
        ip += instruction;
        steps += 1;
    }

    steps
}

fn read_instructions(input: &'static str) -> Vec<i32> {
    let mut cursor = Cursor::new(input);
    let mut instructions: Vec<i32> = Vec::new();

    loop {
        let mut buf = String::new();
        let num_bytes = cursor.read_line(&mut buf).unwrap();

        if num_bytes == 0 {
            break;
        }
        else {
            instructions.push(i32::from_str_radix(buf.trim(), 10).unwrap())
        }
    }

    instructions
}
