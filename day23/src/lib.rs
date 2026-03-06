use std::collections::VecDeque;
use imac::Imac;

pub fn part1(mut network: Vec<Imac>, mut vm255: Imac) -> (i64, i64) {
    // the network should already have IDs for each NIC
    let mut outputs: Vec<VecDeque<i64>> = vec![VecDeque::new(); network.len()];
    
    loop {
        // run each
        for vm in network.iter_mut() {
            vm.run();
        }
        // process output
        for i in 0..network.len() {
            while let Some(j) = network[i].read_out() {
                outputs[i].push_back(j);
            }
        }
        // process inputs
        let mut inputs: Vec<VecDeque<i64>> =
            vec![VecDeque::from(vec![-1i64]); network.len()];
        let mut in255 = VecDeque::from(vec![-1i64]);
        for i in 0..outputs.len() {
            while outputs[i].len() > 0 {
                // outputs should always be in the form of three numbers
                let addr = outputs[i].pop_front().unwrap();
                let x = outputs[i].pop_front().unwrap();
                let y = outputs[i].pop_front().unwrap();
                if addr == 255 {
                    in255.clear();
                    in255.push_back(x);
                    in255.push_back(y);
                } else {
                    if inputs[addr as usize].len() < 2 {
                        inputs[addr as usize].clear();
                    }
                    inputs[addr as usize].push_back(x);
                    inputs[addr as usize].push_back(y);
                }
            }
        }
        // load in inputs
        for i in 0..inputs.len() {
            network[i].load_in(&mut inputs[i]);
        }
        if in255[0] != -1 {
            let x = in255[0];
            let y = in255[1];
            vm255.load_in(&mut in255);
            return (x, y)
        }
    }
}

pub fn part2(mut network: Vec<Imac>, mut vm255: Imac) -> i64 {
    let mut last = -1i64;
    let mut outputs: Vec<VecDeque<i64>> = vec![VecDeque::new(); network.len()];

    loop {
        // run each
        for vm in network.iter_mut() {
            vm.run();
        }
        // process output
        for i in 0..network.len() {
            while let Some(j) = network[i].read_out() {
                outputs[i].push_back(j);
            }
        }
        // process inputs
        let mut quiet = true;
        let mut inputs: Vec<VecDeque<i64>> =
            vec![VecDeque::from(vec![-1i64]); network.len()];
        let mut in255 = VecDeque::from(vec![-1i64]);
        for i in 0..outputs.len() {
            while outputs[i].len() > 0 {
                // outputs should always be in the form of three numbers
                let addr = outputs[i].pop_front().unwrap();
                let x = outputs[i].pop_front().unwrap();
                let y = outputs[i].pop_front().unwrap();
                if addr == 255 {
                    in255.clear();
                    in255.push_back(x);
                    in255.push_back(y);
                } else {
                    quiet = false;
                    if inputs[addr as usize].len() < 2 {
                        inputs[addr as usize].clear();
                    }
                    inputs[addr as usize].push_back(x);
                    inputs[addr as usize].push_back(y);
                }
            }
        }
        // load in inputs
        if in255[0] != -1 {
            let x = in255[0];
            let y = in255[1];
            vm255.load_in(&mut in255);
            if quiet == true {
                if last == y {
                    return y
                } else {
                    last = y;
                    network[0].load_in(&mut VecDeque::from(vec![x, y]));
                    for i in 1..inputs.len() {
                        network[i].load_in(&mut inputs[i]);
                    }
                }
            } else {
                for i in 0..inputs.len() {
                    network[i].load_in(&mut inputs[i]);
                }
            }
        } else {
            for i in 0..inputs.len() {
                network[i].load_in(&mut inputs[i]);
            }
        }
    }
}