use imac::Imac;

pub fn part1(s: &String) -> i64 {
    let p1: Vec<i64> = s.split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();

    let mut prog = Imac::new(&p1, 0, Some(in_q));
    prog.run();
    // TODO count output block tiles (2) after one cycle
    let mut outputs: Vec<i64> = Vec::new();
    while let Some(i) = prog.read_out() {
        outputs.push(i);
    }
    let mut total = 0;
    for i in outputs {
        total += i;
    }

    total
}