use std::io::{self, BufRead};
use std::{string::String, vec::Vec};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines().map(|line| line.unwrap());

    let mut amount_of_valid_pw = 0;

    for line in stdin {
        let rule_pw_split: Vec<String> = line.split(":").map(|x| x.to_string()).collect();
        let index_char_split: Vec<String> = rule_pw_split[0].split(" ").map(|x| x.to_string()).collect();
        let low_high_split = index_char_split[0].split("-").collect::<Vec<&str>>();

        let pw = rule_pw_split[1].trim().chars().collect::<Vec<char>>();
        let character = index_char_split[1].chars().collect::<Vec<char>>()[0];
        let pos1 = low_high_split[0].parse::<u32>().unwrap();
        let pos2 = low_high_split[1].parse::<u32>().unwrap();
        let char1 = pw[pos1 as usize - 1];
        let char2 = pw[pos2 as usize - 1];

        if char1 == character || char2 == character {
            if char1 != char2 {
                amount_of_valid_pw += 1;
            }
        }
    }

    println!("{}", amount_of_valid_pw);
}
