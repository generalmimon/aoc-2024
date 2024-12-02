use std::{collections::HashMap, io};

fn main() {
    let mut left_list = Vec::new();
    let mut right_list_counter: HashMap<u32, u32> = HashMap::new();

    for (line_no, line) in (1_usize..) .zip(io::stdin().lines()) {
        let line = line.unwrap();
        let tokens: Vec<_> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        assert_eq!(2, tokens.len(), "line {line_no}: expected 2 tokens, but got {}", tokens.len());
        let (left, right) = (tokens[0], tokens[1]);
        left_list.push(left);
        let counter = right_list_counter.entry(right).or_default();
        *counter += 1;
    }
    let similarity_score_summands = left_list
        .iter()
        .map(|&v| v * right_list_counter.get(&v).unwrap_or(&0));
    // dbg!(similarity_score_summands.collect::<Vec<_>>());
    let res: u32 = similarity_score_summands.sum();
    println!("{res}");
}
