use std::fs;
use day22::part1;

fn main() {
    let filename = "input22.txt";
    let s = fs::read_to_string(filename)
    .expect("Could not open input file");

    let ans_p1 = part1(&s);
    println!("Part one is {}", ans_p1.0);
}
