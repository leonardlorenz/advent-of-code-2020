use std::io::{self, Read};
use std::string::String;
use std::vec::Vec;

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();

    let numbers: Vec<u32> = buffer
        .split('\n')
        .filter(|&x| x != "")
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{}", calc(numbers).unwrap());
}

fn calc(buffer: Vec<u32>) -> std::option::Option<u32> {
    for x in &buffer {
        for y in &buffer {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    None
}
