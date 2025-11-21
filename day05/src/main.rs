use std::fs;
use imac::Imac;

fn main() {
    let s = fs::read_to_string("input05.txt")
        .expect("Could not read input file");
    let p: Vec<i64> = s.split(',')
        .filter_map(|i| i.trim().parse().ok())
        .collect();
    
    // Part 1
    let cur_p = p.clone();
    let mut prog = Imac::new(&cur_p, 0);
    let p1_out = prog.run(); // input 1
    println!("{}", p1_out);

    // Part2
    let mut prog = Imac::new(&cur_p, 0);
    let p1_out = prog.run(); // input 5
    println!("{}", p1_out);
}