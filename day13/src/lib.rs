use std::collections::VecDeque;
use imac::Imac;

pub fn part1(s: &String) -> i64 {
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
    let mut total = 0;
    for chunk in outputs.chunks_exact(3) {
        if chunk[2] == 2 {
            total += 1;
        }
    }

    total
}

fn find_cross(p: &Imac) -> i64 {
    let mut p_copy = p.clone();
    let mut in_q: VecDeque<i64> = VecDeque::new();
    let mut outputs: Vec<i64> = Vec::new();

    while true {
        // pass in no joy stick movement each time
        in_q.push_back(0);
        p_copy.load_in(&mut in_q);
        // run a step
        p_copy.run();
        // get output
        outputs.clear();
        while let Some(i) = p_copy.read_out() {
            outputs.push(i);
        }
        // check updated ball position
        for chunk in outputs.chunks_exact(3) {
            if chunk[2] == 4 && chunk[1] == 2 {
                return chunk[0].clone()
            }
        }
    }

    panic!("We should return a ball position before this");
}

pub fn part2(s: &String) -> i64 {
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
    let mut total = 0;
    for chunk in outputs.chunks_exact(3) {
        if chunk[2] == 2 {
            total += 1;
        }
    }

    total
}