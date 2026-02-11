use imac::Imac;
use std::collections::VecDeque;

pub fn part1(s: &String) -> i64 {
    let p: Vec<i64> = s.split(',')
    .map(|i| i.trim().parse().unwrap()).collect();
    let vm = Imac::new(&p, 0, Some(VecDeque::new()));
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

pub fn part2(s: &String) -> i64 {
    let p: Vec<i64> = s.split(',')
    .map(|i| i.trim().parse().unwrap()).collect();
    let vm = Imac::new(&p, 0, Some(VecDeque::new()));

    // will have the starting and ending x position for row y
    let mut beam_width: Vec<[i64; 2]> = Vec::new();
    let start_y = 1500;
    let start_x = 1650;
    let mut y = start_y;
    loop {
        let mut beam_start = 0;
        let mut beam_end = 0;
        let mut line_done = false;
        for x in (start_x+(y-start_y))..(start_x+800) {
            let mut cur_vm = vm.clone();
            cur_vm.load_in(&mut VecDeque::from(vec![x, y]));
            cur_vm.run();
            if let Some(pulled) = cur_vm.read_out() {
                if pulled == 1 {
                    if beam_start == 0 {
                        beam_start = x;
                    } else {
                        beam_end = x;
                    }
                } else if beam_end != 0 {
                    line_done = true;
                }
            } else {
                eprintln!("Error: we should always get an output")
            }
            if line_done {
                break;
            }
        }
        beam_width.push([beam_start, beam_end]);
        if y >= start_y + 99 {
            if beam_width[((y - start_y) - 99) as usize][1] -
            beam_width[(y - start_y) as usize][0] >= 99 {
                let ans = (beam_width[((y - start_y) - 99) as usize][1] - 99) * 10000 + (y - 99);
                return ans
            }
        }
        y += 1;
    }
}