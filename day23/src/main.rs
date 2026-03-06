use day23::{part1, part2};
use imac::Imac;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let filename = "input23.txt";
    let s = fs::read_to_string(filename)
    .expect("Could not read input file");
    let s = s.trim().to_string();
    
    let p: Vec<i64> = s.split(',').map(|c| c.parse().unwrap()).collect();
    let mut network: Vec<Imac> = Vec::new();
    for i in 0..50 as i64{
        let vm = Imac::new(&p, 0,
            Some(VecDeque::from(vec![i])));
        network.push(vm);
    }
    let vm255 = Imac::new(&p, 0,
        Some(VecDeque::from(vec![255])));
    
    let net_copy = network.clone();
    let copy255 = vm255.clone();
    let ans_p1 = part1(network, vm255);
    println!("{}", ans_p1.1);

    let ans_p2 = part2(net_copy, copy255);
    println!("{ans_p2}");
}
