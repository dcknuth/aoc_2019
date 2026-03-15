use std::collections::VecDeque;
use std::io::{self, Write};
use std::fs;
use imac::Imac;
use itertools::Itertools;

pub fn part1(s: &String) -> String {
    let p: Vec<i64> = s.split(',').map(|c| c.parse().unwrap()).collect();
    let mut vm = Imac::new(&p, 0, None);
    
    // load up the list of commands to collect everything and get it to
    //  the penultimate room
    let moves = fs::read_to_string("d25_moves.txt")
        .expect("Could not read moves file");
    let mut moves: VecDeque<&str> = moves.split('\n').collect();
    
    // loop of: run the vm, print room/action, get input, load input
    while moves.len() > 0 {
        // run this step
        vm.run();

        // get output and print it
        let mut cur_txt = String::new();
        while let Some(i) = vm.read_out() {
            if let Some(c) = std::char::from_u32(i as u32) {
                cur_txt.push(c);
            } else {
                println!("Invalid ASCII val {i}");
            }
        }
        println!("{cur_txt}");

        // get input
        let input = expand_command(moves.pop_front().unwrap());
        println!("expanded input is {input}");
        let mut input_ints = VecDeque::<i64>::new();
        to_ints(&input, &mut input_ints);

        // load input
        vm.load_in(&mut input_ints);

        if cur_txt.contains("password") {
            return cur_txt
        }
    }
    // one more run to move to penultimate room
    vm.run();
    // drain output
    let mut cur_txt = String::new();
    while let Some(i) = vm.read_out() {
        if let Some(c) = std::char::from_u32(i as u32) {
            cur_txt.push(c);
        } else {
            println!("Invalid ASCII val {i}");
        }
    }
    println!("{cur_txt}");

    // now we should be in the penultimate room with all the stuff and
    //  at the input part. Loop to try all the combinations. Assume output
    //  will lack "heavier" or "lighter" when correct
    let mut done = false;
    let item_list = vec!["fixed point",
        "spool of cat6",
        "monolith",
        "planetoid",
        "hypercube",
        "candy cane",
        "easter egg",
        "ornament"];
    let mut dropped: Vec<&str> = Vec::new();
    let list_len = item_list.len();
    let max_drops = list_len - 1;
    while !done {
        for num_drops in 1..max_drops {
            let combos = item_list.clone().into_iter()
                .combinations(list_len - num_drops);
            for cur_set in combos {
                // get list to drop
                let mut drop: Vec<&str> = Vec::new();
                for i in &item_list {
                    if !cur_set.contains(i) {
                        drop.push(i);
                    }
                }
                // drop unwanted items
                for i in &drop {
                    let input = format!("drop {}", *i);
                    let mut input_ints = VecDeque::<i64>::new();
                    to_ints(&input, &mut input_ints);
                    vm.load_in(&mut input_ints);
                    vm.run();
                    // drain output
                    let mut cur_txt = String::new();
                    while let Some(i) = vm.read_out() {
                        if let Some(c) = std::char::from_u32(i as u32) {
                            cur_txt.push(c);
                        } else {
                            println!("Invalid ASCII val {i}");
                        }
                    }
                }
                // step on scale
                let input = "west";
                let mut input_ints = VecDeque::<i64>::new();
                to_ints(&input, &mut input_ints);
                vm.load_in(&mut input_ints);
                vm.run();
                // get output
                let mut cur_txt = String::new();
                while let Some(i) = vm.read_out() {
                    if let Some(c) = std::char::from_u32(i as u32) {
                        cur_txt.push(c);
                    } else {
                        println!("Invalid ASCII val {i}");
                    }
                }
                if !(cur_txt.contains("heavier") || cur_txt.contains("lighter")) {
                    return cur_txt.to_string()
                }
                // pick up all dropped items
                for i in &drop {
                    let input = format!("take {}", *i);
                    let mut input_ints = VecDeque::<i64>::new();
                    to_ints(&input, &mut input_ints);
                    vm.load_in(&mut input_ints);
                    vm.run();
                    // drain output
                    let mut cur_txt = String::new();
                    while let Some(i) = vm.read_out() {
                        if let Some(c) = std::char::from_u32(i as u32) {
                            cur_txt.push(c);
                        } else {
                            println!("Invalid ASCII val {i}");
                        }
                    }
                }
            }
        }
    }


    "not found".to_string()
}

fn expand_command(s: &str) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();

    match words[0] {
        "w" => return "north".to_string(),
        "a" => return "west".to_string(),
        "s" => return "south".to_string(),
        "d" => return "east".to_string(),
        "i" => return "inv".to_string(),
        "e" => return "exit".to_string(),
        "pu" => return format!("take {}", words[1..].join(" ")),
        "pd" => return format!("drop {}", words[1..].join(" ")),
        _ => {println!("Invalid command {}", words[0]);
            return "exit".to_string()}
    }
}

fn to_ints(inputs: &str, input_ints: &mut VecDeque<i64>) {
    for c in inputs.chars() {
        input_ints.push_back(c as i64);
    }
    input_ints.push_back(10);
}