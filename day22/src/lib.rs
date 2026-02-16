use num_bigint::BigInt;
use num_traits::{Euclid, ToPrimitive};

pub fn part1(s: &String) -> (i64, i64, i64) {
    // Since we already know how to do this from Python we will return
    //  the answer, and the two modulus part we need for part 2

    let mut moves: Vec<&str> = s.split('\n').collect();
    // get rid of the empty at the end
    moves.pop();
    
    // do part 1 and collect the coeffs for part 2
    let (card, a, b) = shuffle(moves, 2019, 10007);

    (card, a, b)
}

fn shuffle(moves: Vec<&str>, mut idx: i64, deck_len: i64) -> (i64, i64, i64) {
    let mut a: BigInt = BigInt::from(1);
    let mut b: BigInt = BigInt::from(0);
    for s in moves {
        if s.contains("new stack") {
            idx = deck_len - 1 - idx;
            a *= -1;
            b = deck_len - 1 - b;
        } else if s.contains("cut ") {
            let cut_n: i64 = s.split(' ').last().unwrap().parse().unwrap();
            idx = (idx - cut_n).rem_euclid(deck_len);
            b -= cut_n;
        } else if s.contains("with increment") {
            let inc: i64 = s.split(' ').last().unwrap().parse().unwrap();
            idx = (idx * inc).rem_euclid(deck_len);
            a *= inc;
            b *= inc;
        } else {
            eprintln!("Unknown shuffle type");
        }
    }
    
    (idx, a.rem_euclid(&BigInt::from(deck_len)).to_i64().unwrap(),
    b.rem_euclid(&BigInt::from(deck_len)).to_i64().unwrap())
}