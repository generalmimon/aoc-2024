// SPDX-FileCopyrightText: 2024 Petr Pucil <petr.pucil@seznam.cz>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io;

fn main() {
    let input = read_input_from_stdin();
    let res = solve(&input, Part::Two);
    println!("{res}");
}

#[derive(Clone, Copy)]
enum Part {
    One,
    Two,
}

impl Part {
    fn is_concat_allowed(self) -> bool {
        match self {
            Part::One => false,
            Part::Two => true,
        }
    }
}

fn solve(input: &[InputEquation], part: Part) -> u64 {
    input
        .iter()
        .filter(|&equ| can_reach_target_result(&equ.operands, equ.target_result, part.is_concat_allowed()))
        .map(|equ| equ.target_result)
        .sum()
}

fn can_reach_target_result(operands: &[u64], target_result: u64, is_concat_allowed: bool) -> bool {
    let (&last_operand, rest_operands) = operands.split_last().unwrap();
    if rest_operands.is_empty() {
        return last_operand == target_result;
    }
    if let Some(addition_rest) = target_result.checked_sub(last_operand) {
        if can_reach_target_result(rest_operands, addition_rest, is_concat_allowed) {
            return true;
        }
    }
    if let Some(multiplication_rest) = target_result.checked_div(last_operand) {
        if target_result % last_operand == 0
            && can_reach_target_result(rest_operands, multiplication_rest, is_concat_allowed)
        {
            return true;
        }
    }
    if is_concat_allowed {
        let nearest_10_pow = nearest_higher_power_of_10(last_operand);
        if target_result % nearest_10_pow == last_operand {
            let concat_rest = target_result / nearest_10_pow;
            if can_reach_target_result(rest_operands, concat_rest, is_concat_allowed) {
                return true;
            }
        }
    }
    false
}

fn nearest_higher_power_of_10(x: u64) -> u64 {
    let x_log10 = x.checked_ilog10().unwrap_or(0);
    10_u64.pow(x_log10 + 1)
}

#[derive(Debug)]
struct InputEquation {
    target_result: u64,
    operands: Vec<u64>,
}

fn read_input_from_stdin() -> Vec<InputEquation> {
    io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let (target_result, operands) = line.split_once(": ").unwrap();
            let target_result = target_result.parse::<u64>().unwrap();
            let operands: Vec<u64> = operands
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            InputEquation {
                target_result,
                operands,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_power_of_10() {
        assert_eq!(nearest_higher_power_of_10(0), 10);
        assert_eq!(nearest_higher_power_of_10(1), 10);
        assert_eq!(nearest_higher_power_of_10(9), 10);
        assert_eq!(nearest_higher_power_of_10(10), 100);
    }
}
