use std::fs;
use std::time::Instant;
use day14::part1;

fn main() {
    let filename = "input14.txt";
    let s = fs::read_to_string(filename).
        expect("Could not find input file");

    let t0 = Instant::now();
    let p1_ans = part1(&s);
    let duration = t0.elapsed();
    println!("Ore amount is {p1_ans} in {:.4?}", duration);
}
