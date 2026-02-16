use std::fs;
use day22::{part1, part2};

fn main() {
    let filename = "input22.txt";
    let s = fs::read_to_string(filename)
    .expect("Could not open input file");

    let ans_p1 = part1(&s);
    println!("Part one is {}", ans_p1.0);

    let ans_p2 = part2(&s);
    println!("Part two is {ans_p2}");
}
