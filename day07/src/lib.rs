use imac::{Imac, State};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Amp1 {
    amp_a: Imac,
    amp_b: Imac,
    amp_c: Imac,
    amp_d: Imac,
    amp_e: Imac,
}

impl Amp1 {
    pub fn new(p: &Vec<i64>, phases: &Vec<i64>) -> Self {
        let amp_a = Imac::new(p, 0,
            Some(VecDeque::from(vec![phases[0]])));
        let amp_b = Imac::new(p, 0,
            Some(VecDeque::from(vec![phases[1]])));
        let amp_c = Imac::new(p, 0,
            Some(VecDeque::from(vec![phases[2]])));
        let amp_d = Imac::new(p, 0,
            Some(VecDeque::from(vec![phases[3]])));
        let amp_e = Imac::new(p, 0,
            Some(VecDeque::from(vec![phases[4]])));
        
        Self { amp_a, amp_b, amp_c, amp_d, amp_e }
    }

    pub fn run_amp(&mut self) -> i64 {
        let mut amp_on = true;
        let mut amp_q: VecDeque<i64> = VecDeque::from(vec![0]);
        while amp_on {
            self.amp_a.load_in(&mut amp_q);
            self.amp_a.run();
            if self.amp_a.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_a.read_out() {
                amp_q.push_back(i);
            }
            self.amp_b.load_in(&mut amp_q);
            self.amp_b.run();
            if self.amp_b.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_b.read_out() {
                amp_q.push_back(i);
            }
            self.amp_c.load_in(&mut amp_q);
            self.amp_c.run();
            if self.amp_c.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_c.read_out() {
                amp_q.push_back(i);
            }
            self.amp_d.load_in(&mut amp_q);
            self.amp_d.run();
            if self.amp_d.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_d.read_out() {
                amp_q.push_back(i);
            }
            self.amp_e.load_in(&mut amp_q);
            self.amp_e.run();
            if self.amp_e.get_state() == State::Ended {
                amp_on = false;
            }
        }
        if let Some(i) = self.amp_e.read_out() {
            return i
        } else {
            panic!("Error in final amp result");
        }
    }

    pub fn run_amp_w_fb(&mut self) -> i64 {
        let mut amp_on = true;
        let mut amp_q: VecDeque<i64> = VecDeque::from(vec![0]);
        while amp_on {
            self.amp_a.load_in(&mut amp_q);
            self.amp_a.run();
            if self.amp_a.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_a.read_out() {
                amp_q.push_back(i);
            }
            self.amp_b.load_in(&mut amp_q);
            self.amp_b.run();
            if self.amp_b.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_b.read_out() {
                amp_q.push_back(i);
            }
            self.amp_c.load_in(&mut amp_q);
            self.amp_c.run();
            if self.amp_c.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_c.read_out() {
                amp_q.push_back(i);
            }
            self.amp_d.load_in(&mut amp_q);
            self.amp_d.run();
            if self.amp_d.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_d.read_out() {
                amp_q.push_back(i);
            }
            self.amp_e.load_in(&mut amp_q);
            self.amp_e.run();
            if self.amp_e.get_state() == State::Ended {
                amp_on = false;
            }
            while let Some(i) = self.amp_e.read_out() {
                amp_q.push_back(i);
            }
        }
        if let Some(i) = amp_q.pop_front() {
            return i
        } else {
            panic!("Error in final amp result");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn test1() {
        let p1: Vec<i64> = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .split(',').filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let signal: i64 = 43210;
        let phases: Vec<i64> = vec![4, 3, 2, 1, 0];
        let mut amp = Amp1::new(&p1, &phases);
        let out_sig = amp.run_amp();
        assert_eq!(signal, out_sig);
    }

    #[test]
    fn test2() {
        let p1: Vec<i64> = concat!("3,23,3,24,1002,24,10,24,1002,23,-1,23,",
            "101,5,23,23,1,24,23,23,4,23,99,0,0")
            .split(',').filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let signal: i64 = 54321;

        let mut max_sig: i64 = -1;
        let mut max_phases: Vec<i64> = vec![0, 0, 0, 0, 0];
        let nums: Vec<i64> = vec![0, 1, 2, 3, 4];
        for phases in nums.iter().cloned().permutations(nums.len()) {
            let mut amp = Amp1::new(&p1, &phases);
            let out_sig = amp.run_amp();
            if out_sig > max_sig {
                max_sig = out_sig;
                max_phases = phases;
            }
        }
        assert_eq!(signal, max_sig);
        assert_eq!(vec![0,1,2,3,4], max_phases);
    }

    #[test]
    fn test3() {
        let p1: Vec<i64> = concat!("3,31,3,32,1002,32,10,32,1001,31,-2,31,",
            "1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")
            .split(',').filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let signal: i64 = 65210;

        let mut max_sig: i64 = -1;
        let mut max_phases: Vec<i64> = vec![0, 0, 0, 0, 0];
        let nums: Vec<i64> = vec![0, 1, 2, 3, 4];
        for phases in nums.iter().cloned().permutations(nums.len()) {
            let mut amp = Amp1::new(&p1, &phases);
            let out_sig = amp.run_amp();
            if out_sig > max_sig {
                max_sig = out_sig;
                max_phases = phases;
            }
        }
        assert_eq!(signal, max_sig);
        assert_eq!(vec![1,0,4,3,2], max_phases);
    }

    #[test]
    fn test4() {
        let p1: Vec<i64> = concat!("3,26,1001,26,-4,26,3,27,1002,27,2,27,1",
            ",27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")
            .split(',').filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let signal: i64 = 139629729;

        let mut max_sig: i64 = -1;
        let mut max_phases: Vec<i64> = vec![0, 0, 0, 0, 0];
        let nums: Vec<i64> = vec![5, 6, 7, 8, 9];
        for phases in nums.iter().cloned().permutations(nums.len()) {
            let mut amp = Amp1::new(&p1, &phases);
            let out_sig = amp.run_amp_w_fb();
            if out_sig > max_sig {
                max_sig = out_sig;
                max_phases = phases;
            }
        }
        assert_eq!(signal, max_sig);
        assert_eq!(vec![9,8,7,6,5], max_phases);
    }

    #[test]
    fn test5() {
        let p1: Vec<i64> = concat!("3,52,1001,52,-5,52,3,53,1,52,56,54,",
            "1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53",
            ",1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,",
            "1005,56,6,99,0,0,0,0,10")
            .split(',').filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let signal: i64 = 18216;

        let mut max_sig: i64 = -1;
        let mut max_phases: Vec<i64> = vec![0, 0, 0, 0, 0];
        let nums: Vec<i64> = vec![5, 6, 7, 8, 9];
        for phases in nums.iter().cloned().permutations(nums.len()) {
            let mut amp = Amp1::new(&p1, &phases);
            let out_sig = amp.run_amp_w_fb();
            if out_sig > max_sig {
                max_sig = out_sig;
                max_phases = phases;
            }
        }
        assert_eq!(signal, max_sig);
        assert_eq!(vec![9,7,8,5,6], max_phases);
    }
}