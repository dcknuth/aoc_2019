fn parse_input(s: &String) -> Vec<Vec<u64>> {
    let lines: Vec<&str> = s.lines().collect();
    let mut conversions: Vec<Vec<u64>> = vec![vec![0; lines.len()+1]; lines.len()+1];
    // insert a unit conversion for ORE
    conversions[0][0] = 1;
    conversions[1][1] = 1;
    // we need to know the index/positions of ORE and FUEL, so we set those
    let mut items: Vec<&str> = vec!["ORE", "FUEL"];
    for l in lines {
        let mut cur_i = 0;
        let parts = l.split_once(" => ").unwrap();
        let inputs: Vec<&str> = parts.0.split(", ").collect();
        let output = parts.1.split_once(' ').unwrap();
        match items.iter().position(|&s| s == output.1) {
            Some(index) => cur_i = index,
            None => {
                cur_i = items.len();
                items.push(output.1);
            },
        }
        conversions[cur_i][cur_i] = output.0.parse().unwrap();
        for item in inputs {
            let sub_parts = item.split_once(' ').unwrap();
            match items.iter().position(|&s| s == sub_parts.1) {
                Some(index) => conversions[cur_i][index] = sub_parts.0.parse().unwrap(),
                None => {
                    conversions[cur_i][items.len()] = sub_parts.0.parse().unwrap();
                    items.push(sub_parts.1);
                },
            }
        }
    }
    
    conversions
}

fn expand_level(l: Vec<(usize, u64)>, ct: &Vec<Vec<u64>>,
    remainders: &mut Vec<u64>) -> Vec<(usize, u64)> {
    let mut expanded_l: Vec<(usize, u64)> = Vec::new();

    for (item, q) in l {
        let mut need = q;
        if remainders[item] >= need {
            remainders[item] = remainders[item] - need;
            continue;
        }
        need -= remainders[item];
        remainders[item] = 0;
        let production_q = ct[item][item];
        let steps = need.div_ceil(production_q);
        for sub_item in 0..ct[item].len() {
            if (sub_item == item && sub_item != 0) || ct[item][sub_item] == 0 {
                continue;
            }
            match expanded_l.iter().position(|i| i.0 == sub_item) {
                Some(index) => {
                    expanded_l[index] = (expanded_l[index].0,
                        expanded_l[index].1 + ct[item][sub_item] * steps);
                },
                None => expanded_l.push((sub_item, ct[item][sub_item] * steps)),
            }
        }
        remainders[item] += production_q * steps - need;
    }
    
    expanded_l
}

pub fn part1(s: &String) -> u64 {
    let ct = parse_input(s);
    let mut remainders = vec![0; ct.len()];
    let mut l = vec![(1, 1)];
    loop {
        if l.len() == 1 && l[0].0 == 0 {
            break
        }
        l = expand_level(l, &ct, &mut remainders);
    }

    l[0].1
}

pub fn part2(s: &String) -> u64 {
    let ct = parse_input(s);
    let mut remainders = vec![0; ct.len()];
    let mut fuel_min: u64 = 1;
    let mut fuel_max: u64 = 10000000;
    

    while fuel_min != fuel_max {
        let cur_fuel: u64 = fuel_min.midpoint(fuel_max);
        if cur_fuel == fuel_min {
            break
        }
        let mut l = vec![(1, cur_fuel)];
        loop {
            if l.len() == 1 && l[0].0 == 0 {
                break
            }
            l = expand_level(l, &ct, &mut remainders);
        }
        let cur_ore = l[0].1;
        if cur_ore < 1000000000000u64 {
            fuel_min = cur_fuel;
        } else {
            fuel_max = cur_fuel;
        }
    }

    fuel_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let s = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL".to_string();
        let c = parse_input(&s);
        assert_eq!(c.len(), 7);
    }

    #[test]
    fn test_expand_level() {
        let s = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL".to_string();
        let c = parse_input(&s);
        let l = vec![(1, 1)];
        let el = expand_level(l, &c, &mut vec![0; c.len()]);
        assert_eq!(el.len(), 2);
    }

    #[test]
    fn test_part1() {
        let s = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL".to_string();
        let total_ore = part1(&s);
        assert_eq!(total_ore, 31);
    }

    #[test]
    fn test_part1_2() {
        let s = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX".to_string();
        let total_ore = part1(&s);
        assert_eq!(total_ore, 2210736);
    }
}