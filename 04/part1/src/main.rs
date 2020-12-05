use std::io::{self, Read};
use std::vec::Vec;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let raw_passports = input.split("\n\n").collect::<Vec<&str>>();

    let mut passports: Vec<Vec<&str>> = vec![];
    for l in raw_passports {
        let a = l.replace("\n", " ");
        let b = a.split(" ").collect::<Vec<&str>>();
        passports.push(b);
    }
}
