
use regex::Regex;
use std::collections::VecDeque;
use crate::utils::read_lines;

pub fn run(fname: &str) {
    parse_input(fname);
}

// #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct CPU {
    x: i32,
    cycle: i32,
    xvec: Vec<i32>, /* x value vec */
    cvec: VecDeque<i32>  /* cycle vec */
}

impl CPU {
    pub fn new(cvec_arg: Vec<i32>) -> CPU {
        let mut cvecdeq: VecDeque<i32> = VecDeque::from(cvec_arg);
        cvecdeq.make_contiguous().sort();
        CPU{
            x: 1,
            cycle: 0,
            xvec: vec![],
            cvec: cvecdeq
        }
    }
    fn cycle_incr(&mut self, n: i32) {
        for _ix in 0..n {
            self.cycle += 1;
            if self.cvec.len() > 0 {
                if self.cycle == self.cvec[0] {
                    self.cvec.pop_front();
                    self.xvec.push(self.x);
                }
            }
        }
    }
    pub fn addx(&mut self, v: i32) {
        self.x += v;
        self.cycle_incr(2);
    }
    pub fn noop(&mut self) { self.cycle_incr(1); }
}

fn parse_input(fname: &str) {
    let addx_repat = Regex::new(r"^addx (-?\d+)").unwrap();
    let noop_repat = Regex::new(r"^noop").unwrap();
    println!("Parsing {fname}");
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ok_line) = line {
                println!("{ok_line}");
                if let Some(caps) = addx_repat.captures(&ok_line) {
                    let vstr = caps.get(1).unwrap().as_str();
                    let v: i32 = vstr.parse::<i32>().unwrap();
                    println!("addx, v = {v}");
                }
                else if noop_repat.is_match(&ok_line) {
                    println!("noop!");
                }
            }
        }
    }
 
}