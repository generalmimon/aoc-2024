use std::io;

fn main() {
    let input = read_input_from_stdin();
    let output = solve(input);
    println!("{output}");
}

type Input = Vec<Vec<u32>>;

fn solve(input: Input) -> usize {
    input
        .iter()
        .filter(|v| is_safe(v))
        .count()
}

fn is_safe(report: &[u32]) -> bool {
    let mut it = report.iter();
    let Some(&prev) = it.next() else {
        return true;
    };
    let mut prev = prev;

    let mut increasing = true;
    let mut decreasing = true;
    for &v in it {
        match prev.cmp(&v) {
            std::cmp::Ordering::Less => decreasing = false,
            std::cmp::Ordering::Greater => increasing = false,
            std::cmp::Ordering::Equal => return false,
        }
        if !(1..=3).contains(&prev.abs_diff(v)) {
            return false;
        }
        prev = v;
    }
    increasing || decreasing
}

fn read_input_from_stdin() -> Input {
    io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .map(|x| x.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect())
        .collect()
}
