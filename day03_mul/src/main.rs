// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: MIT

use std::io::{self, Read};

use regex::Regex;

fn main() {
    let input = read_input_from_stdin();
    let res = solve(&input, Part::Two);
    println!("{res}")
}

enum Part {
    One,
    Two,
}

fn solve(input: &str, part: Part) -> i32 {
    match part {
        Part::One => solve_part1(input),
        Part::Two => solve_part2(input),
    }
}

fn solve_part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let matches = re.captures_iter(&input);
    let mut res = 0;
    for m in matches {
        let args = [m.get(1), m.get(2)]
            .map(|x| x.unwrap().as_str().parse::<i32>().unwrap());
        res += args[0] * args[1];
    }
    res
}

fn solve_part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(do(?:n't)?)\(\)").unwrap();
    let matches = re.captures_iter(&input);
    let mut res = 0;
    let mut mul_enabled = true;
    for m in matches {
        if let Some(cond_instr) = m.get(3).map(|x| x.as_str()) {
            match cond_instr {
                "do" => mul_enabled = true,
                "don't" => mul_enabled = false,
                _ => unreachable!()
            }
        } else if mul_enabled {
            let args = [m.get(1), m.get(2)]
                .map(|x| x.unwrap().as_str().parse::<i32>().unwrap());
            res += args[0] * args[1];
        }
    }
    res
}

fn read_input_from_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}
