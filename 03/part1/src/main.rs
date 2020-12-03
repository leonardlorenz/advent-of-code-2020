use std::io::{self, BufRead};
use std::vec::Vec;

fn check_if_valid(loc: [usize; 2], map: &Vec<Vec<char>>) -> std::result::Result<[usize; 2], &str> {
    match map.get(loc[0]) {
        None => Err("y out of bounds"),
        Some(v) => {
            match v.get(loc[1]) {
                None => Err("x out of bounds"),
                Some(_) => Ok([loc[0], loc[1]])
            }
        }
    }
}

fn next_move(loc: [usize; 2], map: &Vec<Vec<char>>) -> std::result::Result<[usize; 2], &str> {
    let mut next_x = loc[1] + 3;
    // if next_x goes out of bounds, yeet back to start
    if next_x >= map[0].len() {
        next_x -= map[0].len();
    }
    let next_y = loc[0] + 1;
    //println!("{:?}", loc);

    check_if_valid([next_y, next_x], map)
}

fn recursion(count: u16, iteration: u16, loc: [usize; 2], map: &Vec<Vec<char>>) -> (u16, u16) {
    match next_move(loc, map) {
        Err(e) => {
            println!("{}", e);
            (count, iteration)
        }
        Ok(next_loc) => {
            match map[next_loc[0]][next_loc[1]] {
                '#' => recursion(count + 1, iteration + 1, next_loc, map),
                '.' => recursion(count, iteration + 1, next_loc, map),
                _ => panic!("character is neither '#' or '.'")
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let map: Vec<Vec<char>> = stdin
        .lock()
        .lines()
        .map(|line| {
            line
            .unwrap()
            .chars()
            .collect()
        }).collect::<Vec<Vec<char>>>();

    let count;
    if map[0][0] == '#' {
        count = 1;
    } else {
        count = 0
    }

    let result = recursion(count, 0, [0,0], &map);
    println!("count: {}\niteration: {}", result.0, result.1);
}
