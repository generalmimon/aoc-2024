// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: MIT

use std::io;

fn main() {
    let input = read_input_from_stdin();
    let output = solve(input, Part::Two);
    println!("{output}");
    // for report in &input {
    //     println!("{report:?}: part1 {}, part2 {}", if is_safe_part1(&report) { "S" } else { "U" }, if is_safe_part2(&report) { "S" } else { "U" });
    // }
}

enum Part {
    One,
    Two,
}

type Input = Vec<Vec<i32>>;

fn solve(input: Input, part: Part) -> usize {
    let is_safe_fn: fn(&[i32]) -> bool =
        match part {
            Part::One => is_safe_part1,
            Part::Two => is_safe_part2,
        };

    input
        .iter()
        .filter(|v| is_safe_fn(v))
        .count()
}

fn is_safe_part1(report: &[i32]) -> bool {
    let diffs = report_to_diffs(&report);
    let Some(&first) = diffs.first() else {
        return true;
    };
    let expected_sign = first.signum();
    diffs.iter().all(|&diff| diff.signum() == expected_sign && is_diff_safe(diff))
}

fn is_safe_part2(report: &[i32]) -> bool {
    if is_safe_part1(report) {
        return true;
    }

    let mut safe_candidate: Vec<i32> = vec![0; report.len() - 1];
    for i_remove in 0..report.len() {
        for (dst, &src) in safe_candidate[..i_remove].iter_mut().zip(report[..i_remove].iter()) {
            *dst = src;
        }
        for (dst, &src) in safe_candidate[i_remove..].iter_mut().zip(report[i_remove + 1..].iter()) {
            *dst = src;
        }
        if is_safe_part1(&safe_candidate) {
            return true;
        }
    }
    false
}

fn report_to_diffs(report: &[i32]) -> Vec<i32> {
    report
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect()
}

fn is_diff_safe(diff: i32) -> bool {
    (1..=3).contains(&diff.abs())
}

fn read_input_from_stdin() -> Input {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|x| x.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn report_diffs() {
        assert_eq!(vec![-1, -2, -2, -1], report_to_diffs(&[7, 6, 4, 2, 1]));
        assert_eq!(vec![1, 5, 1, 1], report_to_diffs(&[1, 2, 7, 8, 9]));
    }
}
