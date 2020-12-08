use std::io::{self, Read};
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let opcodes = input
        .split('\n')
        .map(|instr| instr.split(' ').collect())
        .collect::<Vec<Vec<&str>>>();

    let mut accumulator: i32 = 0;
    let mut instr_pointer: usize = 0;
    let mut visited_instructions: Vec<usize> = vec![];
    while instr_pointer < opcodes.len() {
        if visited_instructions.contains(&instr_pointer) {
            break;
        } else {
            visited_instructions.push(instr_pointer);
        }

        let instr = opcodes[instr_pointer][0];
        let value = opcodes[instr_pointer][1];

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
    println!("{}", accumulator);
}
