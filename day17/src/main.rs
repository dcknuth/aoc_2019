use std::fs;
use imac::Imac;
use std::collections::VecDeque;
use day17::{part1, part2};

fn main() {
    let filename = "input17.txt";
    let s = fs::read_to_string(filename)
        .expect("Could not read input file");

    let p: Vec<i64> = s.split(',')
    .filter_map(|i| i.trim().parse::<i64>().ok())
    .collect();
    let mut vm = Imac::new(&p, 0, Some(VecDeque::new()));

    let ans_p1 = part1(&mut vm);
    println!("{:?}", ans_p1.0);

    // we now have the map and the robot position from part 1 and we know
    //  it starts facing up (yes, carrying this from the Python version)
    // make new VM with 2 in the 0 position
    let mut vm = Imac::new(&p, 0, Some(VecDeque::new()));
    vm.set_idx(0, 2);
    vm.run();
    let ans_p2 = part2(&mut vm, &ans_p1.1, &ans_p1.2);
    println!("{ans_p2}");
}
