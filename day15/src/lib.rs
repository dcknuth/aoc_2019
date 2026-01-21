use std::collections::{VecDeque, HashMap, HashSet};
use imac::Imac;

pub fn part1(s: &String) -> (u64, i8, i8, HashMap<(i8, i8), Vec<(i8, i8)>>) {
    // Take the puzzle input
    // Find the moves to the Ox tank: first return value
    // Find the coords of the Ox tank: second and third return values
    // Make graph: last return value, each coord has Vec of connections

    // make vm
    let p: Vec<i64> = s.split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();
    let vm = Imac::new(&p, 0, Some(VecDeque::new()));

    // setup other variables
    let mut visited: HashSet<(i8, i8)> = HashSet::new();
    let mut g: HashMap<(i8, i8), Vec<(i8, i8)>> = HashMap::new();
    let mut moves = 1u64;
    let directions: Vec<(i8, i8)> = vec![(1, 0), (-1, 0), (0, -1), (0, 1)];
    let mut ox_y = 0i8;
    let mut ox_x = 0i8;
    let cur_pos = (49i8, 49i8);  // assume start is middle of 100x100
    // prime our loop
    visited.insert(cur_pos);
    let mut positions = vec![(vm, cur_pos)];

    while positions.len() > 0 {
        let mut new_positions: Vec<(Imac, (i8, i8))> = Vec::new();
        for (vm, (y, x)) in &positions[..] {
            // TODO try each unvisited adjacent position
            for i in 0..directions.len() {
                if visited.contains(&(y+directions[i].0, x+directions[i].1)) {
                    continue;
                }
                let mut cur_vm = vm.clone();
                cur_vm.load_in(&mut VecDeque::from([(i+1) as i64]));
                cur_vm.run();
                if let Some(j) = cur_vm.read_out() {
                    match j {
                        0 => {
                            visited.insert((y+directions[i].0, x+directions[i].1));
                        },
                        1 => {
                            visited.insert((y+directions[i].0, x+directions[i].1));
                            // add edges
                            g.entry((*y, *x)).or_insert_with(Vec::new)
                                .push((y+directions[i].0, x+directions[i].1));
                            g.insert((y+directions[i].0, x+directions[i].1), vec![(*y, *x)]);
                            // push on new_pos list
                            new_positions.push((cur_vm, (y+directions[i].0, x+directions[i].1)));
                        },
                        2 => {
                            // record Ox position
                            ox_y = y+directions[i].0;
                            ox_x = x+directions[i].1;
                            visited.insert((y+directions[i].0, x+directions[i].1));
                            // add edges
                            g.entry((*y, *x)).or_insert_with(Vec::new)
                                .push((y+directions[i].0, x+directions[i].1));
                            g.insert((y+directions[i].0, x+directions[i].1), vec![(*y, *x)]);
                            // push on new_pos list
                            new_positions.push((cur_vm, (y+directions[i].0, x+directions[i].1)));
                        },
                        _ => eprintln!("Invalid move response!"),
                    }
                }
            }
        }
        if ox_y == 0 && ox_x == 0 {
            moves += 1;
        }
        positions = new_positions;
    }
    
    (moves, ox_y, ox_x, g)
}

pub fn part2(g: HashMap<(i8, i8), Vec<(i8, i8)>>, ox_y: i8, ox_x: i8) -> u64 {
    // move outward from the Ox tank location one step at a time until every
    //  location has been visited
    let mut seconds = 0;
    let mut positions = vec![(ox_y, ox_x)];
    let mut visited = vec![(ox_y, ox_x)];

    while positions.len() > 0 {
        let mut new_positions: Vec<(i8, i8)> = Vec::new();
        for cur_pos in &positions[..] {
            for &next_pos in g.get(cur_pos).unwrap() {
                if !visited.contains(&next_pos) {
                    new_positions.push(next_pos);
                    visited.push(next_pos);
                }
            }
        }
        positions = new_positions;
        seconds += 1;
    }

    seconds - 1
}