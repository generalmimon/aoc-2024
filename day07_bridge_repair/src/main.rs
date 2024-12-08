use std::io;

fn main() {
    let input = read_input_from_stdin();
    let res = solve_part1(&input);
    println!("{res}");
}

fn solve_part1(input: &[InputEquation]) -> u64 {
    input
        .iter()
        .filter(|&equ| can_reach_target_result(&equ.operands, equ.target_result))
        .map(|equ| equ.target_result)
        .sum()
}

fn can_reach_target_result(operands: &[u64], target_result: u64) -> bool {
    let (&last_operand, rest_operands) = operands.split_last().unwrap();
    if rest_operands.is_empty() {
        return last_operand == target_result;
    }
    if let Some(addition_rest) = target_result.checked_sub(last_operand) {
        if can_reach_target_result(rest_operands, addition_rest) {
            return true;
        }
    }
    if let Some(multiplication_rest) = target_result.checked_div(last_operand) {
        if target_result % last_operand == 0
            && can_reach_target_result(rest_operands, multiplication_rest)
        {
            return true;
        }
    }
    false
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
