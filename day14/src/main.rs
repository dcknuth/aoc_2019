use std::fs;
use std::time::Instant;
use day14::{part1, part2, part2_bs};

fn main() {
    let filename = "input14.txt";
    let s = fs::read_to_string(filename).
        expect("Could not find input file");

    let t0 = Instant::now();
    let p1_ans = part1(&s);
    let duration = t0.elapsed();
    println!("Ore amount is {p1_ans} in {:.4?}", duration);

    // let t0 = Instant::now();
    // let p2_ans = part2(&s);
    // let duration = t0.elapsed();
    // println!("Fuel produced is {p2_ans} in {:.4?}", duration);

    let t0 = Instant::now();
    let p2_ans = part2_bs(&s);
    let duration = t0.elapsed();
    println!("Fuel produced is {p2_ans} in {:.4?}", duration);
}
