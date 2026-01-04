use std::collections::VecDeque;
use imac::{Imac, State};

pub fn part1(s: &String) -> i64 {
    let p1: Vec<i64> = s.split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();

    let in_q = VecDeque::from([1i64]);
    let mut prog = Imac::new(&p1, 0, Some(in_q));
    prog.run();
    let mut outputs: Vec<i64> = Vec::new();
    while let Some(i) = prog.read_out() {
        outputs.push(i);
    }
    let mut total = 0;
    for chunk in outputs.chunks_exact(3) {
        if chunk[2] == 2 {
            total += 1;
        }
    }

    total
}

fn find_cross(g: &Game) -> i64 {
    let mut g_copy = g.clone();
    // pass in no joy stick movement each time (by setting target)
    g_copy.paddle_target = -1;

    loop {
        // run a step
        g_copy.step();
        if g_copy.mac.get_state() == imac::State::Ended {
            return -1
        }
        // check updated ball position
        if g_copy.ball[1] == 18 {
            return g_copy.ball[0].clone()
        }
    }

    panic!("We should return a ball x position before this");
}

// Just counted from the Python version
const W: usize = 36;
const H: usize = 21;

#[derive(Debug, Clone)]
struct Game {
    screen: Vec<Vec<i64>>,
    ball: [i64; 2],
    ball_dir: [i64; 2],
    paddle: [i64; 2],
    paddle_target: i64,
    score: i64,
    mac: Imac,
}

impl Game {
    fn new(int_vec: Vec<i64>) -> Self {
        let mut new_vec = int_vec.clone();
        new_vec[0] = 2; // set for "free play"
        let mac = Imac::new(&new_vec, 0, Some(VecDeque::new()));

        Self {
            screen: vec![vec![0; W]; H],
            ball: [-1, -1],
            ball_dir: [0, 0],
            paddle: [-1, -1],
            paddle_target: -1,
            score: 0,
            mac,
        }
    }

    fn step(&mut self) -> State {
        self.mac.run();
        // should have output and be waiting for input
        // get the output
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = self.mac.read_out() {
            outputs.push(i);
        }
        for chunk in outputs.chunks_exact(3) {
            if chunk[0] < 0 {
                // this is the score
                self.score = chunk[2];
            } else {
                // this is something on the screen
                self.screen[chunk[1] as usize][chunk[0] as usize] = chunk[2];
                if chunk[2] == 3 {
                    // paddle
                    self.paddle[0] = chunk[0];
                    self.paddle[1] = chunk[1];
                } else if chunk[2] == 4 {
                    // ball
                    self.ball_dir[0] = chunk[0] - self.ball[0];
                    self.ball_dir[1] = chunk[1] - self.ball[1];
                    self.ball[0] = chunk[0];
                    self.ball[1] = chunk[1];
                }
            }
        }
        // now give input for paddle
        if self.paddle[0] == self.paddle_target || self.paddle_target == -1 {
            self.mac.load_in(&mut VecDeque::from([0i64]));
        } else if self.paddle[0] < self.paddle_target {
            self.mac.load_in(&mut VecDeque::from([1i64]));
        } else {
            self.mac.load_in(&mut VecDeque::from([-1i64]));
        }
        
        self.mac.get_state()
    }

}

pub fn part2(s: &String) -> i64 {
    let p1: Vec<i64> = s.split(',')
        .filter_map(|s| s.trim().parse::<i64>().ok())
        .collect();

    let mut game = Game::new(p1);
    while game.mac.get_state() != imac::State::Ended {
        game.step();
        // println!("Ball at {:?},{:?} Paddle at {:?},{:?}", game.ball[0],
        //     game.ball[1], game.paddle[0], game.paddle[1]);
        let mut cur_target = game.paddle_target.clone();
        if game.ball[1] == game.paddle[1] - 1 {
            // find next paddle cross to target
            cur_target = find_cross(&game);
        }
        game.paddle_target = cur_target;
    }

    game.score
}