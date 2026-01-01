use std::fs;
use std::time::Instant;
use day13::part1;

fn main() {
    let filename = "input13.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not read input file");

    let t0 = Instant::now();
    let ans_p1 = part1(&s);
    let duration = t0.elapsed();
    println!("Part 1 is {ans_p1} in {duration:.4?}");
}
