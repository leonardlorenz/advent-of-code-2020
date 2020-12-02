use std::io::{self, BufRead};
use std::{string::String, vec::Vec};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock().lines().map(|line| line.unwrap());

    let mut amount_of_valid_pw = 0;

    for line in stdin {
        let meta: Vec<String> = line.split(":").map(|x| x.to_string()).collect();
        let rules = &meta[0];
        let pw = &meta[1];
        let meta: Vec<String> = rules.split(" ").map(|x| x.to_string()).collect();
        let low_high = &meta[0];
        let character = &meta[1].chars().collect::<Vec<char>>()[0];
        let meta: Vec<String> = low_high.split("-").map(|x| x.to_string()).collect();
        let low = &meta[0].parse::<u8>().unwrap();
        let high = &meta[1].parse::<u8>().unwrap();

        let amount = pw.chars().filter(|c| c == character).collect::<Vec<char>>().len();

        if amount >= *low as usize && amount <= *high as usize {
            amount_of_valid_pw += 1;
        }
    }

    println!("{}", amount_of_valid_pw);
}
