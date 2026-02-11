use imac::Imac;
use std::collections::VecDeque;

pub fn part1(s: &String) -> i64 {
    let p: Vec<i64> = s.split(',')
    .map(|i| i.trim().parse().unwrap()).collect();
    let mut vm = Imac::new(&p, 0, Some(VecDeque::new()));
    let mut total = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut cur_vm = vm.clone();
            cur_vm.load_in(&mut VecDeque::from(vec![x, y]));
            cur_vm.run();
            if let Some(pulled) = cur_vm.read_out() {
                if pulled == 1 {
                    total += 1;
                }
            } else {
                eprintln!("Error: we should always get an output")
            }
        }
    }

    total
}