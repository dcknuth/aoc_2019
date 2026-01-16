use std::collections::HashMap;

fn parse_input(s: &String) -> HashMap<&str, HashMap<&str, u64>> {
    let mut conversions: HashMap<&str, HashMap<&str, u64>> = HashMap::new();
    let lines: Vec<&str> = s.lines().collect();
    for l in lines {
        let parts = l.split_once(" => ").unwrap();
        let inputs: Vec<&str> = parts.0.split(", ").collect();
        let output = parts.1.split_once(' ').unwrap();
        let cur_conversion = conversions.entry(output.1)
            .or_insert(HashMap::from([(output.1, output.0.parse().unwrap())]));
        for item in inputs {
            let sub_parts = item.split_once(' ').unwrap();
            cur_conversion.insert(sub_parts.1, sub_parts.0.parse().unwrap());
        }
    }
    // insert a unit conversion for ORE
    conversions.insert("ORE", HashMap::from([("ORE", 1)]));

    conversions
}

fn expand_level<'a>(l: HashMap<&'a str, u64>, ct: &HashMap<&str, HashMap<&'a str, u64>>,
    remainders: &mut HashMap<&'a str, u64>) -> HashMap<&'a str, u64> {
    let mut expanded_l: HashMap<&str, u64> = HashMap::new();

    for (item, q) in l {
        let mut need = q;
        let have = remainders.entry(item).or_insert(0);
        if *have >= need {
            *have = *have - need;
            continue;
        }
        need -= *have;
        *have = 0;
        let sub_items = ct.get(item).unwrap();
        let &production_q = sub_items.get(item).unwrap();
        let steps = need.div_ceil(production_q);
        for &sub_item in sub_items.keys() {
            if sub_item == item && sub_item != "ORE" {
                continue;
            }
            let next_item = expanded_l.entry(sub_item).or_insert(0);
            *next_item += sub_items.get(sub_item).unwrap() * steps;
        }
        *have += production_q * steps - need;
    }
    
    expanded_l
}

fn ore_equiv(item: &str, need: u64, ct: HashMap<&str, HashMap<&str, u64>>)
    -> u64 {
    // Turns the item and amount needed of that item into the amount of
    // ore needed to produce it
    let mut remainders = HashMap::new();
    let mut l = HashMap::from([(item, need)]);
    loop {
        if l.len() == 1 && *l.keys().next().unwrap() == "ORE" {
            break
        }
        l = expand_level(l, &ct, &mut remainders);
    }
    
    *l.get("ORE").unwrap()
}

pub fn part1(s: &String) -> u64 {
    let c = parse_input(s);
    let total_ore = ore_equiv("FUEL", 1, c);

    total_ore
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
        let l = HashMap::from([("FUEL", 1)]);
        let el = expand_level(l, &c, &mut HashMap::new());
        assert_eq!(el.len(), 2);
    }

    #[test]
    fn test_ore_equiv() {
        let s = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL".to_string();
        let c = parse_input(&s);
        let total_ore = ore_equiv("FUEL", 1, c);
        assert_eq!(total_ore, 31);
    }

    #[test]
    fn test_ore_equiv2() {
        let s = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL".to_string();
        let c = parse_input(&s);
        let total_ore = ore_equiv("FUEL", 1, c);
        assert_eq!(total_ore, 165);
    }

    #[test]
    fn test_ore_equiv3() {
        let s = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_string();
        let c = parse_input(&s);
        let total_ore = ore_equiv("FUEL", 1, c);
        assert_eq!(total_ore, 13312);
    }

    #[test]
    fn test_ore_equiv4() {
        let s = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF".to_string();
        let c = parse_input(&s);
        let total_ore = ore_equiv("FUEL", 1, c);
        assert_eq!(total_ore, 180697);
    }

    #[test]
    fn test_ore_equiv5() {
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
        let c = parse_input(&s);
        let total_ore = ore_equiv("FUEL", 1, c);
        assert_eq!(total_ore, 2210736);
    }
}