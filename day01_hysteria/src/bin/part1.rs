// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: MIT

use std::io;

fn main() {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for (line_no, line) in (1_usize..) .zip(io::stdin().lines()) {
        let line = line.unwrap();
        let tokens: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        assert_eq!(2, tokens.len(), "line {line_no}: expected 2 tokens, but got {}", tokens.len());
        let (left, right) = (tokens[0], tokens[1]);
        left_list.push(left);
        right_list.push(right);
    }
    left_list.sort_unstable();
    right_list.sort_unstable();
    let distances = left_list.iter()
        .zip(right_list.iter())
        .map(|(&l, &r)| l.abs_diff(r));
    // dbg!(distances.collect::<Vec<_>>());
    let res: u32 = distances.sum();
    println!("{res}");
}
