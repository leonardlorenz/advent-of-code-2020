use std::io::{self, Read};
use std::vec::Vec;

fn run(code: Vec<Vec<&str>>) -> (bool, i32) {
    let mut accumulator: i32 = 0;
    let mut instr_pointer: usize = 0;
    let mut visited_instructions: Vec<usize> = vec![];
    let mut looped = false;

    while instr_pointer < code.len() {
        if visited_instructions.contains(&instr_pointer) {
            looped = true;
            break;
        } else {
            visited_instructions.push(instr_pointer);
        }

        let instr = code[instr_pointer][0];
        let value = code[instr_pointer][1];

        match instr {
            "nop" => {
                instr_pointer += 1;
                continue;
            }
            "jmp" => {
                instr_pointer += value.parse::<i32>().unwrap() as usize;
            }
            "acc" => {
                accumulator += value.parse::<i32>().unwrap();
                instr_pointer += 1;
            }
            _ => panic!(),
        }
    }
    (looped, accumulator)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let opcodes = input
        .split('\n')
        .map(|instr| instr.split(' ').collect::<Vec<&str>>())
        .filter(|instr| instr.len() != 1)
        .collect::<Vec<Vec<&str>>>();

    for (i, code) in opcodes.iter().enumerate() {
        match code[0] {
            "nop" => {
                let mut test_opcodes = opcodes.clone();
                test_opcodes[i][0] = "jmp";
                let result = run(test_opcodes);
                if !result.0 {
                    println!("{}", result.1);
                }
            }
            "jmp" => {
                let mut test_opcodes = opcodes.clone();
                test_opcodes[i][0] = "nop";
                let result = run(test_opcodes);
                if !result.0 {
                    println!("{}", result.1);
                }
            }
            _ => continue,
        }
    }
}
