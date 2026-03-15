use std::fs;
use day25::part1;

fn main() {
    let filename = "input25.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not read input file");
    let s = s.trim().to_string();
    
    let out_str = part1(&s);
    println!("{out_str}");
}
