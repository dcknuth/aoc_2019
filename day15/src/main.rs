use std::fs;
use std::time::Instant;
use day15::{part1, part2};

fn main() {
    let filename = "input15.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not read input file");

    let t0 = Instant::now();
    let ans_p1 = part1(&s);
    let duration = t0.elapsed();
    println!("Moves to Ox tank is {} in {:.4?}", ans_p1.0, duration);

    let t0 = Instant::now();
    let ans_p2 = part2(ans_p1.3, ans_p1.1, ans_p1.2);
    let duration = t0.elapsed();
    println!("Seconds to full Ox is {ans_p2} in {duration:.4?}");
}
