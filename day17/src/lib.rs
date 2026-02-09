use imac::Imac;
use std::collections::VecDeque;

pub fn part1(vm: &mut Imac) -> (i64, Vec<Vec<i64>>, [i64; 2]) {
    // where we will put the robot info
    let mut robot = [0; 2];
    // run the program
    vm.run();
    // read out the scaffold view
    let mut view: Vec<Vec<i64>> = Vec::new();
    let mut cur_line: Vec<i64> = Vec::new();
    while let Some(i) = vm.read_out() {
        if i == 10 {
            view.push(cur_line.clone());
            cur_line.clear();
        } else {
            cur_line.push(i);
        }
    }
    // pop last empty line
    view.pop();

    // now count intersections
    let dirs: Vec<[i64; 2]> = vec![[-1, 0], [0, 1], [1, 0], [0, -1]];
    let mut total = 0;
    for y in 0..view.len() as i64 {
        for x in 0..view[0].len() as i64 {
            if view[y as usize][x as usize] == 94 {
                robot[0] = y;
                robot[1] = x;
            }
            if y > 0 && y < view.len() as i64 - 1 && x > 0 &&
                x < view[0].len() as i64 - 1 &&
                view[y as usize][x as usize] == 35 {
                let mut n = 0;
                for [dy, dx] in &dirs {
                    let ty = y + dy;
                    let tx = x + dx;
                    if view[ty as usize][tx as usize] == 35 {
                        n += 1;
                    }
                }
                if n > 2 {
                    total += y * x;
                }
            }
        }
    }

    (total, view, robot)
}

pub fn part2(vm: &mut Imac, view: &Vec<Vec<i64>>, robot: &[i64; 2]) -> i64 {
    // We know there is a naive path to the end following mandatory turns
    //  so we find this first
    // We will leave as ASCII ints for now R = 82 L = 76
    let mut steps: Vec<u8> = Vec::new();
    let dirs: Vec<[i64; 2]> = vec![[-1, 0], [0, 1], [1, 0], [0, -1]];
    let mut cur_r = robot.clone();
    let mut cur_dir: usize = 0;

    // we know the obvious end is y=20, x=30 from the Python printout
    let mut straight_for = 0;
    while cur_r[0] != 20 || cur_r[1] != 30 {
        // try to move the current direction
        let ty = cur_r[0] + dirs[cur_dir][0];
        let tx = cur_r[1] + dirs[cur_dir][1];
        if ty > -1 && tx > -1 && ty < view.len() as i64 &&
        tx < view[0].len() as i64 && view[ty as usize][tx as usize] == 35 {
            cur_r[0] = ty;
            cur_r[1] = tx;
            straight_for += 1;
            continue;
        }
        // try a right turn next
        cur_dir = (cur_dir + 1) % 4;
        let ty = cur_r[0] + dirs[cur_dir][0];
        let tx = cur_r[1] + dirs[cur_dir][1];
        if ty > -1 && tx > -1 && ty < view.len() as i64 &&
        tx < view[0].len() as i64 && view[ty as usize][tx as usize] == 35 {
            if straight_for != 0 {
                steps.push(straight_for);
                straight_for = 0;
            }
            steps.push(82);
            continue;
        }
        // must be a left. Three rights make a left and we don't
        //  have to deal with the non-modulus behavior of %
        cur_dir = (cur_dir + 2) % 4;
        let ty = cur_r[0] + dirs[cur_dir][0];
        let tx = cur_r[1] + dirs[cur_dir][1];
        if ty > -1 && tx > -1 && ty < view.len() as i64 &&
        tx < view[0].len() as i64 && view[ty as usize][tx as usize] == 35 {
            if straight_for != 0 {
                steps.push(straight_for);
                straight_for = 0;
            }
            steps.push(76);
            continue;
        }
        eprintln!("Error: we seem to have an issue generating the path");
    }
    // we will end in a straight length
    if straight_for != 0 {
        steps.push(straight_for);
    }
    
    // We could do this next part manually/visually, but since we did that
    //  in the Python version, let's try to do it programmatically here
    // We need to try to break into sections
    // Something at the beginning with possible repeats, something in the
    //  middle with possible repeats, and something at the end with possible
    //  repeats
    // To bound the search, we will assume that every pattern repeats at
    //  least once, but not more than 6 times
    // Each sub-pattern should be at least 3 long, but not more than 10
    // Find possible As
    let a_options: Vec<&[u8]> = (6..11)
    .filter_map(|a| {
        let pattern = &steps[0..a];
        // Check if pattern exists later in steps
        steps[a..].windows(pattern.len())
            .any(|window| window == pattern)
            .then_some(pattern)
    })
    .collect();
    // Find possible Cs (at the end)
    let c_options: Vec<&[u8]> = (6..11)
    .filter_map(|c| {
        let pattern = &steps[steps.len()-c..];
        // Check if pattern exists later in steps
        steps[0..steps.len()-c].windows(pattern.len())
            .any(|window| window == pattern)
            .then_some(pattern)
    })
    .collect();

    // Now loop assuming a B follows the first A
    for a in a_options {
        let b_options: Vec<&[u8]> = (6..11)
        .filter_map(|b| {
            let pattern = &steps[a.len()..a.len()+b];
            // Check if pattern exists later in steps
            steps[a.len()+b..].windows(pattern.len())
                .any(|window| window == pattern)
                .then_some(pattern)
        })
        .collect();
        for b in b_options {
            for c in &c_options {
                if let Some((pm, ap, bp, cp)) = test_set(a, b, c, &steps) {
                    // if we find one, enter into the intcode vm
                    // println!("pm = {:?}", pm);
                    // println!("ap = {:?}", ap);
                    // println!("bp = {:?}", bp);
                    // println!("cp = {:?}", cp);
                    // should be waiting for input
                    let mut pmq = VecDeque::from(pm);
                    vm.load_in(&mut pmq);
                    vm.run();
                    let mut apq = VecDeque::from(ap);
                    vm.load_in(&mut apq);
                    vm.run();
                    let mut bpq = VecDeque::from(bp);
                    vm.load_in(&mut bpq);
                    vm.run();
                    let mut cpq = VecDeque::from(cp);
                    vm.load_in(&mut cpq);
                    vm.run();
                    // now, do we want video feed
                    vm.load_in(&mut VecDeque::from(vec![b'n' as i64, 10]));
                    vm.run();
                    // there should now be output that ends with the dust
                    //  quantity
                    let mut final_out: Vec<i64> = Vec::new();
                    while let Some(i) = vm.read_out() {
                        final_out.push(i);
                    }
                    // println!("Final out is\n{:?}", final_out);

                    return final_out[final_out.len() - 1]
                }
            }
        }
    }

    // should return inside the loop, but if it does not
    0
}

fn test_set(a: &[u8], b: &[u8], c: &[u8], steps: &Vec<u8>) ->
Option<(Vec<i64>, Vec<i64>, Vec<i64>, Vec<i64>)> {
    // first test all three for length requirement
    let mut ap: Vec<i64> = Vec::new();
    let mut bp: Vec<i64> = Vec::new();
    let mut cp: Vec<i64> = Vec::new();
    for &i in a {
        if i == 10 {
            ap.push(49);
            ap.push(48);
            ap.push(44);
        } else if i == 12 {
            ap.push(49);
            ap.push(50);
            ap.push(44);
        } else if i < 10 {
            ap.push(i as i64 + 48);
            ap.push(44);
        } else {
            ap.push(i as i64);
            ap.push(44);
        }
    }
    // pop the last ','
    ap.pop();
    if ap.len() > 20 {
        return None
    }

    for &i in b {
        if i == 10 {
            bp.push(49);
            bp.push(48);
            bp.push(44);
        } else if i == 12 {
            bp.push(49);
            bp.push(50);
            bp.push(44);
        } else if i < 10 {
            bp.push(i as i64 + 48);
            bp.push(44);
        } else {
            bp.push(i as i64);
            bp.push(44);
        }
    }
    // pop the last ','
    bp.pop();
    if bp.len() > 20 {
        return None
    }

    for &i in c {
        if i == 10 {
            cp.push(49);
            cp.push(48);
            cp.push(44);
        } else if i == 12 {
            cp.push(49);
            cp.push(50);
            cp.push(44);
        } else if i < 10 {
            cp.push(i as i64 + 48);
            cp.push(44);
        } else {
            cp.push(i as i64);
            cp.push(44);
        }
    }
    // pop the last ','
    cp.pop();
    if cp.len() > 20 {
        return None
    }

    // Now we need to see if we can create a matching set
    let mut mp: Vec<i64> = Vec::new();
    let mut offset = a.len();
    mp.push(b'A' as i64);
    mp.push(b',' as i64);
    while offset < steps.len() {
        // try a
        if steps[offset..].starts_with(a) {
            offset += a.len();
            mp.push(b'A' as i64);
            mp.push(b',' as i64);
            continue;
        }
        // try b
        if steps[offset..].starts_with(b) {
            offset += b.len();
            mp.push(b'B' as i64);
            mp.push(b',' as i64);
            continue;
        }
        // try c
        if steps[offset..].starts_with(c) {
            offset += c.len();
            mp.push(b'C' as i64);
            mp.push(b',' as i64);
            continue;
        }
        return None
    }
    // pop the last ','
    mp.pop();
    
    if mp.len() > 20 {
        return None
    }
    // add newlines
    mp.push(10);
    ap.push(10);
    bp.push(10);
    cp.push(10);
    Some((mp, ap, bp, cp))
}