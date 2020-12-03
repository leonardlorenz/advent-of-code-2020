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

fn next_move(slope: [usize; 2], loc: [usize; 2], map: &Vec<Vec<char>>) -> std::result::Result<[usize; 2], &str> {
    let mut next_x = loc[1] + slope[0];
    // if next_x goes out of bounds, yeet back to start
    if next_x >= map[0].len() {
        next_x -= map[0].len();
    }
    let next_y = loc[0] + slope[1];

    check_if_valid([next_y, next_x], map)
}

fn recursion(count: u16, iteration: u16, slope: [usize; 2], loc: [usize; 2], map: &Vec<Vec<char>>) -> (u16, u16) {
    match next_move(slope, loc, map) {
        Err(_) => {
            (count, iteration)
        }
        Ok(next_loc) => {
            match map[next_loc[0]][next_loc[1]] {
                '#' => recursion(count + 1, iteration + 1, slope, next_loc, map),
                '.' => recursion(count, iteration + 1, slope, next_loc, map),
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

    const slopes: [[usize; 2]; 5] = [
        [1, 1],
        [3, 1],
        [5, 1],
        [7, 1],
        [1, 2]
    ];

    let mut results: [u32; slopes.len()] = [0; slopes.len()];

    for (i, slope) in slopes.iter().enumerate() {
        let result = recursion(count, 0, *slope, [0,0], &map);
        println!("count: {}, iterations: {}", result.0, result.1);
        results[i] = result.0 as u32;
    }

    println!("result: {}", results.iter().product::<u32>());
}
