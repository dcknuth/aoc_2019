use imac::Imac;
use std::collections::VecDeque;

pub fn part1(s: &String) -> i64 {
    let p: Vec<i64> = s.strip_suffix('\n').unwrap().split(',')
    .map(|i| i.parse().unwrap()).collect();
    let mut vm = Imac::new(&p, 0, Some(VecDeque::new()));
    
    vm.run();
    // get rid of the prompt
    while let Some(_i) = vm.read_out() {}
    
    // format our springscript
    let ss_text = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J";
    let mut ss: VecDeque<i64> = VecDeque::new();
    for s in ss_text.split('\n') {
        for &c in s.as_bytes() {
            ss.push_back(c as i64);
        }
        ss.push_back(10);
    }
    for &c in "WALK".as_bytes() {
        ss.push_back(c as i64);
    }
    ss.push_back(10);

    // load and run our springscript
    vm.load_in(&mut ss);
    vm.run();
    let mut outputs: Vec<i64> = Vec::new();
    while let Some(i) = vm.read_out() {
        outputs.push(i);
    }

    outputs[outputs.len()-1]
}

pub fn part2(s: &String) -> i64 {
    let p: Vec<i64> = s.strip_suffix('\n').unwrap().split(',')
    .map(|i| i.parse().unwrap()).collect();
    let mut vm = Imac::new(&p, 0, Some(VecDeque::new()));
    
    vm.run();
    // get rid of the prompt
    while let Some(_i) = vm.read_out() {}
    
    // format our springscript
    let ss_text = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J";
    let mut ss: VecDeque<i64> = VecDeque::new();
    for s in ss_text.split('\n') {
        for &c in s.as_bytes() {
            ss.push_back(c as i64);
        }
        ss.push_back(10);
    }
    for &c in "RUN".as_bytes() {
        ss.push_back(c as i64);
    }
    ss.push_back(10);

    // load and run our springscript
    vm.load_in(&mut ss);
    vm.run();
    let mut outputs: Vec<i64> = Vec::new();
    while let Some(i) = vm.read_out() {
        outputs.push(i);
    }

    outputs[outputs.len()-1]
}