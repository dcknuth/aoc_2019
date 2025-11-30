use std::fs;
use std::collections::VecDeque;
use imac::Imac;

fn main() {
    let s = fs::read_to_string("input09.txt")
        .expect("Could not read input file");
    let p1: Vec<i64> = s.split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();
    let in_q = VecDeque::from([1i64]);
    let mut prog = Imac::new(&p1, 0, Some(in_q));

    prog.run();

    let mut outputs: Vec<i64> = Vec::new();
    while let Some(i) = prog.read_out() {
        outputs.push(i);
    }
    println!("BOOST keycode is: {}", outputs[0]);

    let in_q = VecDeque::from([2i64]);
    let mut prog = Imac::new(&p1, 0, Some(in_q));

    prog.run();

    let mut outputs: Vec<i64> = Vec::new();
    while let Some(i) = prog.read_out() {
        outputs.push(i);
    }
    println!("Coords is: {}", outputs[0]);
}
