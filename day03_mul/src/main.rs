use std::io::{self, Read};

use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let matches = re.captures_iter(&input);
    let mut res = 0;
    for m in matches {
        let args = [m.get(1), m.get(2)]
            .map(|x| x.unwrap().as_str().parse::<i32>().unwrap());
        res += args[0] * args[1];
    }
    println!("{res}")
}
