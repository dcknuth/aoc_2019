use std::fs;
use imac::Imac;
use itertools::iproduct;

fn main() {
    let s = fs::read_to_string("input02.txt")
        .expect("Could not read input file");
    let p: Vec<i64> = s.split(',')
        .filter_map(|i| i.trim().parse().ok())
        .collect();
    
    // Part 1
    let mut cur_p = p.clone();
    cur_p[1] = 12;
    cur_p[2] = 2;
    let mut prog = Imac::new(&cur_p, 0);
    prog.run();
    println!("Value at 0 is {}", prog.get_idx(0));

    // Part2
    for (noun, verb) in iproduct!(0..100, 0..100) {
        cur_p[1] = noun;
        cur_p[2] = verb;
        let mut prog = Imac::new(&cur_p, 0);
        prog.run();
        if prog.get_idx(0) == 19690720 {
            println!("Part two answer is {}", 100 * noun + verb);
            break;
        }
    }
}
