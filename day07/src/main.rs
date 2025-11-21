use std::fs;
use imac::Imac;
use day07::Amp1;
use itertools::Itertools;

fn main() {
    let s = fs::read_to_string("input07.txt")
        .expect("Could not read input file");
    let p1: Vec<i64> = s.split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();
    
    let mut max_sig: i64 = -1;
    let mut max_phases: Vec<i64> = vec![0, 0, 0, 0, 0];
    let nums: Vec<i64> = vec![0, 1, 2, 3, 4];
    for phases in nums.iter().cloned().permutations(nums.len()) {
        let mut amp = Amp1::new(&p1, &phases);
        let out_sig = amp.run_amp();
        if out_sig > max_sig {
            max_sig = out_sig;
            max_phases = phases;
        }
    }
    
    println!("Part1 Max thrust: {max_sig} with sequence {:?}", max_phases);

    let mut max_sig: i64 = -1;
    let mut max_phases: Vec<i64> = vec![0, 0, 0, 0, 0];
    let nums: Vec<i64> = vec![5, 6, 7, 8, 9];
    for phases in nums.iter().cloned().permutations(nums.len()) {
        let mut amp = Amp1::new(&p1, &phases);
        let out_sig = amp.run_amp_w_fb();
        if out_sig > max_sig {
            max_sig = out_sig;
            max_phases = phases;
        }
    }

    println!("Part2 Max thrust: {max_sig} with sequence {:?}", max_phases);
}
