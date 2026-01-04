use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Ended,
    NotStarted,
    Running,
    Waiting,
}

#[derive(Debug, Clone)]
pub struct Imac {
    p: Vec<i64>,
    index: usize,
    rb_offset: i64,
    in_q: VecDeque<i64>,
    out_q: VecDeque<i64>,
    state: State,
}

impl Imac {
    pub fn new(int_vec: &Vec<i64>, i: usize,
        inputs: Option<VecDeque<i64>>) -> Self {
        let mut p = int_vec.clone();
        p.resize(10000, 0);
        let index = i;
        let rb_offset = 0;
        let in_q = inputs.unwrap_or_else(VecDeque::new);
        let out_q = VecDeque::new();
        let state = State::NotStarted;
        Self { p, index, rb_offset, in_q, out_q, state }
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
        match str_op.chars().nth(2) {
            Some('0') => arg1 = self.p[self.p[self.index+1] as usize],
            Some('1') => arg1 = self.p[self.index+1],
            Some('2') => arg1 =
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in add arg1 at index: {}", self.index)
        }
        match str_op.chars().nth(1) {
            Some('0') => arg2 = self.p[self.p[self.index+2] as usize],
            Some('1') => arg2 = self.p[self.index+2],
            Some('2') => arg2 =
                self.p[(self.rb_offset + self.p[self.index+2]) as usize],
            _ => panic!("Unknown mode in add arg2 at index: {}", self.index)
        }
        match str_op.chars().nth(0) {
            Some('0') => dest = self.p[self.index+3] as usize,
            Some('1') => {
                panic!("Write dest can't be immediate mode add index: {}",
                self.index);
            },
            Some('2') => dest =
                (self.rb_offset + self.p[self.index+3]) as usize,
            _ => panic!("Unknown mode in add arg3 at index: {}", self.index)
        }
        self.p[dest] = arg1 + arg2;
        self.index += 4;
    }
    fn mul(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        let dest: usize;
        match str_op.chars().nth(2) {
            Some('0') => arg1 = self.p[self.p[self.index+1] as usize],
            Some('1') => arg1 = self.p[self.index+1],
            Some('2') => arg1 =
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in mul arg1 at index: {}", self.index)
        }
        match str_op.chars().nth(1) {
            Some('0') => arg2 = self.p[self.p[self.index+2] as usize],
            Some('1') => arg2 = self.p[self.index+2],
            Some('2') => arg2 =
                self.p[(self.rb_offset + self.p[self.index+2]) as usize],
            _ => panic!("Unknown mode in mul arg2 at index: {}", self.index)
        }
        match str_op.chars().nth(0) {
            Some('0') => dest = self.p[self.index+3] as usize,
            Some('1') => {
                panic!("Write dest can't be immediate mode mul index: {}",
                self.index);
            },
            Some('2') => dest =
                (self.rb_offset + self.p[self.index+3]) as usize,
            _ => panic!("Unknown mode in mul arg3 at index: {}", self.index)
        }
        self.p[dest] = arg1 * arg2;
        self.index += 4;
    }
    fn iin(&mut self, str_op: &String) {
        let dest: usize;
        match str_op.chars().nth(2) {
            Some('0') => dest = self.p[self.index+1] as usize,
            Some('1') => {
                panic!("Write dest can't be immediate mode iin index: {}",
                self.index);
            },
            Some('2') => dest =
                (self.rb_offset + self.p[self.index+1]) as usize,
            _ => panic!("Unknown mode in iin dest at index: {}", self.index)
        }
        if let Some(i) = self.in_q.pop_front() {
            self.p[dest] =  i;
            self.index += 2;
        } else {
            self.state = State::Waiting;
        }
    }
    fn iout(&mut self, str_op: &String) {
        let arg1: i64;
        match str_op.chars().nth(2) {
            Some('0') => arg1 = self.p[self.p[self.index+1] as usize],
            Some('1') => arg1 = self.p[self.index+1],
            Some('2') => arg1 =
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in iout arg1 at index: {}", self.index)
        }
        self.index += 2;
        self.out_q.push_back(arg1);
    }
    fn jit(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        match str_op.chars().nth(2) {
            Some('0') => arg1 = self.p[self.p[self.index+1] as usize],
            Some('1') => arg1 = self.p[self.index+1],
            Some('2') => arg1 =
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in jit arg1 at index: {}", self.index)
        }
        if arg1 == 0 {
            self.index += 3;
            return
        }
        match str_op.chars().nth(1) {
            Some('0') => arg2 = self.p[self.p[self.index+2] as usize],
            Some('1') => arg2 = self.p[self.index+2],
            Some('2') => arg2 =
                self.p[(self.rb_offset + self.p[self.index+2]) as usize],
            _ => panic!("Unknown mode in jit arg2 at index: {}", self.index)
        }
        self.index = arg2 as usize;
    }
    fn jif(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        match str_op.chars().nth(2) {
            Some('0') => arg1 = self.p[self.p[self.index+1] as usize],
            Some('1') => arg1 = self.p[self.index+1],
            Some('2') => arg1 =
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in jif arg1 at index: {}", self.index)
        }
        if arg1 != 0 {
            self.index += 3;
            return
        }
        match str_op.chars().nth(1) {
            Some('0') => arg2 = self.p[self.p[self.index+2] as usize],
            Some('1') => arg2 = self.p[self.index+2],
            Some('2') => arg2 =
                self.p[(self.rb_offset + self.p[self.index+2]) as usize],
            _ => panic!("Unknown mode in jif arg2 at index: {}", self.index)
        }
        self.index = arg2 as usize;
    }
    fn lt(&mut self, str_op: &String) {
        let arg1: i64;
        let arg2: i64;
        let dest: usize;
        match str_op.chars().nth(2) {
            Some('0') => arg1 = self.p[self.p[self.index+1] as usize],
            Some('1') => arg1 = self.p[self.index+1],
            Some('2') => arg1 =
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in lt arg1 at index: {}", self.index)
        }
        match str_op.chars().nth(1) {
            Some('0') => arg2 = self.p[self.p[self.index+2] as usize],
            Some('1') => arg2 = self.p[self.index+2],
            Some('2') => arg2 =
                self.p[(self.rb_offset + self.p[self.index+2]) as usize],
            _ => panic!("Unknown mode in lt arg2 at index: {}", self.index)
        }
        match str_op.chars().nth(0) {
            Some('0') => dest = self.p[self.index+3] as usize,
            Some('1') => {
                panic!("Write dest can't be immediate mode. Index: {}",
                self.index);
            },
            Some('2') => dest =
                (self.rb_offset + self.p[self.index+3]) as usize,
            _ => panic!("Unknown mode in lt arg3 at index: {}", self.index)
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
        match str_op.chars().nth(2) {
            Some('0') => arg1 = self.p[self.p[self.index+1] as usize],
            Some('1') => arg1 = self.p[self.index+1],
            Some('2') => arg1 =
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in eq arg1 at index: {}", self.index)
        }
        match str_op.chars().nth(1) {
            Some('0') => arg2 = self.p[self.p[self.index+2] as usize],
            Some('1') => arg2 = self.p[self.index+2],
            Some('2') => arg2 =
                self.p[(self.rb_offset + self.p[self.index+2]) as usize],
            _ => panic!("Unknown mode in eq arg2 at index: {}", self.index)
        }
        match str_op.chars().nth(0) {
            Some('0') => dest = self.p[self.index+3] as usize,
            Some('1') => {
                panic!("Write dest can't be immediate mode. Index: {}",
                self.index);
            },
            Some('2') => dest =
                (self.rb_offset + self.p[self.index+3]) as usize,
            _ => panic!("Unknown mode in eq arg3 at index: {}", self.index)
        }
        if arg1 == arg2 {
            self.p[dest] = 1;
        } else {
            self.p[dest] = 0;
        }
        self.index += 4;
    }
    fn rbo(&mut self, str_op: &String) {
        match str_op.chars().nth(2) {
            Some('0') => self.rb_offset += self.p[self.p[self.index+1] as usize],
            Some('1') => self.rb_offset += self.p[self.index+1],
            Some('2') => self.rb_offset +=
                self.p[(self.rb_offset + self.p[self.index+1]) as usize],
            _ => panic!("Unknown mode in rbo at index: {}", self.index)
        }
        self.index += 2;
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
                3 => self.iin(&str_op),
                4 => self.iout(&str_op),
                5 => self.jit(&str_op),
                6 => self.jif(&str_op),
                7 => self.lt(&str_op),
                8 => self.eq(&str_op),
                9 => self.rbo(&str_op),
                _ => { self.state = State::Ended;
                    println!("Error: Unknown operator {}", self.p[self.index]);
                    println!("       At address: {}", self.index);
                    },
            }
        }
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

        let p_out: String = prog.p_out().chars().take(output.len()-1).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-1).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-1).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-1).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-1).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-1).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-3).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-3).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-1).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-2).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-2).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-2).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-2).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-2).collect();
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

        let p_out: String = prog.p_out().chars().take(output.len()-2).collect();
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
    fn test_rbo1() {
        let input = "109,10,99";
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, None);

        prog.run();

        assert_eq!(prog.rb_offset, 10i64);
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

        let p_out: String = prog.p_out().chars().take(output.len()-5).collect();
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
    fn test_day09() {
        // Input should equal output for this test
        let input = concat!("109,1,204,-1,1001,100,1,100,1008,100,16,101,",
            "1006,101,0,99");
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        let out_str = outputs.iter()
            .map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        assert_eq!(input, out_str);

        let input = "1102,34915192,34915192,7,4,7,99,0";
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        assert_eq!(outputs[0], 1219070632396864);

        let input = "104,1125899906842624,99";
        let test_q = VecDeque::from([8i64]);
        let p_in: Vec<i64> = input.split(',')
            .filter_map(|s| s.trim().parse::<i64>().ok())
            .collect();
        let mut prog = Imac::new(&p_in, 0, Some(test_q));

        prog.run();

        let mut outputs: Vec<i64> = Vec::new();
        while let Some(i) = prog.read_out() {
            outputs.push(i);
        }
        assert_eq!(outputs[0], 1125899906842624);
    }
}
