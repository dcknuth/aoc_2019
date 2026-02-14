use std::fs;
use day21::{part1, part2};

fn main() {
    let filename = "input21.txt";
    let s = fs::read_to_string(filename)
    .expect("Could not read input file");

    let ans_p1 = part1(&s);
    println!("{ans_p1}");

    let ans_p2 = part2(&s);
    println!("{ans_p2}");
}
