use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ended,
    NotStarted,
    Running,
    Waiting,
}

#[derive(Debug)]
pub struct Imac {
    p: Vec<i64>,
    index: usize,
    in_q: VecDeque<i64>,
    out_q: VecDeque<i64>,
    state: State,
}

impl Imac {
    pub fn new(int_vec: &Vec<i64>, i: usize,
        inputs: Option<VecDeque<i64>>) -> Self {
        let p = int_vec.clone();
        let index = i;
        let in_q = inputs.unwrap_or_else(VecDeque::new);
        let out_q = VecDeque::new();
        let state = State::NotStarted;
        Self { p, index, in_q, out_q, state }
    }
    pub fn load_in(&mut self, inputs: &mut VecDeque<i64>) {
        while let Some(i) = inputs.pop_front() {
            self.in_q.push_back(i);
        }
    }
    pub fn read_out(&mut self) -> Option<i64> {
        self.out_q.pop_front()
    }
    pub fn get_idx(&self, i: usize) -> i64 {
        self.p[i]
    }
    pub fn p_out(&self) -> String {
        self.p.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",")
    }
    pub fn get_state(&self) -> State {
        self.state
    }

    fn add(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        let dest: usize;
        if str_op.chars().nth(2) == Some('0') {
            arg1 = self.p[self.p[self.index+1] as usize] as i64;
        } else {
            arg1 = self.p[self.index+1];
        }
        if str_op.chars().nth(1) == Some('0') {
            arg2 = self.p[self.p[self.index+2] as usize] as i64;
        } else {
            arg2 = self.p[self.index+2];
        }
        if str_op.chars().nth(0) == Some('0') {
            dest = self.p[self.index+3] as usize;
        } else {
            panic!("Write locations can't be immediate mode (add). Index: {}",
                self.index);
        }
        self.p[dest] = arg1 + arg2;
        self.index += 4;
    }
    fn mul(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        let dest: usize;
        if str_op.chars().nth(2) == Some('0') {
            arg1 = self.p[self.p[self.index+1] as usize] as i64;
        } else {
            arg1 = self.p[self.index+1];
        }
        if str_op.chars().nth(1) == Some('0') {
            arg2 = self.p[self.p[self.index+2] as usize] as i64;
        } else {
            arg2 = self.p[self.index+2];
        }
        if str_op.chars().nth(0) == Some('0') {
            dest = self.p[self.index+3] as usize;
        } else {
            panic!("Write locations can't be immediate mode. Index: {}",
                self.index);
        }
        self.p[dest] = arg1 * arg2;
        self.index += 4;
    }
    fn iin(&mut self) {
        let dest = self.p[self.index+1] as usize;
        // let mut input = String::new();
        // TODO remove this after updating for day07
        // io::stdin().read_line(&mut input)
        //     .expect("Failed to read in line");
        // self.p[dest] = input.trim().parse().unwrap();
        if let Some(i) = self.in_q.pop_front() {
            self.p[dest] =  i;
            self.index += 2;
        } else {
            self.state = State::Waiting;
        }
    }
    fn iout(&mut self, str_op: &String) {
        let arg1: i64;
        if str_op.chars().nth(2) == Some('0') {
            arg1 = self.p[self.p[self.index+1] as usize] as i64;
        } else {
            arg1 = self.p[self.index+1];
        }
        self.index += 2;
        // TODO remove this after updating for day07
        // format!("{}", arg1)
        self.out_q.push_back(arg1);
    }
    fn jit(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        if str_op.chars().nth(2) == Some('0') {
            arg1 = self.p[self.p[self.index+1] as usize] as i64;
        } else {
            arg1 = self.p[self.index+1];
        }
        if arg1 == 0 {
            self.index += 3;
            return
        }
        if str_op.chars().nth(1) == Some('0') {
            arg2 = self.p[self.p[self.index+2] as usize] as i64;
        } else {
            arg2 = self.p[self.index+2];
        }
        self.index = arg2 as usize;
    }
    fn jif(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        if str_op.chars().nth(2) == Some('0') {
            arg1 = self.p[self.p[self.index+1] as usize] as i64;
        } else {
            arg1 = self.p[self.index+1];
        }
        if arg1 != 0 {
            self.index += 3;
            return
        }
        if str_op.chars().nth(1) == Some('0') {
            arg2 = self.p[self.p[self.index+2] as usize] as i64;
        } else {
            arg2 = self.p[self.index+2];
        }
        self.index = arg2 as usize;
    }
    fn lt(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        let dest: usize;
        if str_op.chars().nth(2) == Some('0') {
            arg1 = self.p[self.p[self.index+1] as usize] as i64;
        } else {
            arg1 = self.p[self.index+1];
        }
        if str_op.chars().nth(1) == Some('0') {
            arg2 = self.p[self.p[self.index+2] as usize] as i64;
        } else {
            arg2 = self.p[self.index+2];
        }
        if str_op.chars().nth(0) == Some('0') {
            dest = self.p[self.index+3] as usize;
        } else {
            panic!("Write locations can't be immediate mode. Index: {}",
                self.index);
        }
        if arg1 < arg2 {
            self.p[dest] = 1;
        } else {
            self.p[dest] = 0;
        }
        self.index += 4;
    }
    fn eq(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        let dest: usize;
        if str_op.chars().nth(2) == Some('0') {
            arg1 = self.p[self.p[self.index+1] as usize] as i64;
        } else {
            arg1 = self.p[self.index+1];
        }
        if str_op.chars().nth(1) == Some('0') {
            arg2 = self.p[self.p[self.index+2] as usize] as i64;
        } else {
            arg2 = self.p[self.index+2];
        }
        if str_op.chars().nth(0) == Some('0') {
            dest = self.p[self.index+3] as usize;
        } else {
            panic!("Write locations can't be immediate mode. Index: {}",
                self.index);
        }
        if arg1 == arg2 {
            self.p[dest] = 1;
        } else {
            self.p[dest] = 0;
        }
        self.index += 4;
    }

    pub fn run(&mut self) {
        self.state = State::Running;
        while self.state == State::Running {
            let str_op = format!("{:05}", self.p[self.index]);
            let cur_op: i64 = str_op[3..].parse().unwrap();
            match cur_op {
                99 => self.state = State::Ended,
                1 => self.add(&str_op),
                2 => self.mul(&str_op),
                3 => self.iin(),
                4 => self.iout(&str_op),
                5 => self.jit(&str_op),
                6 => self.jif(&str_op),
                7 => self.lt(&str_op),
                8 => self.eq(&str_op),
                _ => { self.state = State::Ended;
                    println!("Error: Unknown operator {}", self.p[self.index]);
                    println!("       At address: {}", self.index);
                    },
            }
        }
        // Remove after updating for day07
        // let p_str = self.p.iter()
        //     .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        // let all_out = outputs.join(",");

        // format!("{},{}", p_str, all_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        let input = "1,0,0,0,99";
        let output = "2,0,0,0,99,";
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, None);
        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_add2() {
        let input = "1,1,1,4,99,5,6,0,99";
        let output = "30,1,1,4,2,5,6,0,99,";
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, None);

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_mul() {
        let input = "2,3,0,3,99";
        let output = "2,3,0,6,99,";
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, None);

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_mul2() {
        let input = "2,4,4,5,99,0";
        let output = "2,4,4,5,99,9801,";
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, None);

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_day02() {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";
        let output = "3500,9,10,70,2,3,11,0,99,30,40,50,";
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, None);

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_iin() {
        let input = "3,0,99";
        let output = "10,0,99,";
        let test_q = VecDeque::from([10i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_iout() {
        let input = "4,2,99";
        let output = "4,2,99,99";
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, None);

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_inout() {
        let input = "3,0,4,0,99";
        let output = "10,0,4,0,99,10";
        let test_q = VecDeque::from([10i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_im0() {
        let input = "1002,4,3,4,33";
        let output = "1002,4,3,4,99,";
        let test_q = VecDeque::from([10i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_cmp1() {
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        let output = "3,9,8,9,10,9,4,9,99,1,8,1";
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_cmp2() {
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let output = "3,9,7,9,10,9,4,9,99,0,8,0";
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_cmp3() {
        let input = "3,3,1108,-1,8,3,4,3,99";
        let output = "3,3,1108,1,8,3,4,3,99,1";
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_cmp4() {
        let input = "3,3,1107,-1,8,3,4,3,99";
        let output = "3,3,1107,0,8,3,4,3,99,0";
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_jmp1() {
        let input = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
        let output = "3,12,6,12,15,1,13,14,13,4,13,99,1,1,1,9,1";
        let test_q = VecDeque::from([1i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_jmp2() {
        let input = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
        let output = "3,3,1105,1,9,1101,0,0,12,4,12,99,1,1";
        let test_q = VecDeque::from([1i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }

    #[test]
    fn test_day05() {
        let input = concat!("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,",
            "20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,",
            "1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
        let output = concat!("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006",
            ",20,31,1106,0,36,98,1000,8,1002,21,125,20,4,20,1105,1,46,104,",
            "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99,1000");
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let p_out = prog.p_out();
        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        let p_out = format!("{},{}", p_out, out_str);
        assert_eq!(output, p_out);
    }
}
