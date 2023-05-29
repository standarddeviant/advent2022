
use regex::Regex;
use std::collections::VecDeque;
use crate::utils::read_lines;

pub fn run(fname: &str) {
    let part1_cvec: Vec<i32> = Vec::from([
        20, 60, 100, 140, 180, 220
    ]);
    let p1 = parse_input(fname, part1_cvec);
    println!("p1.ssum = {}", p1.ssum);
}

// #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct CPU {
    x: i32,         /* X reg value */
    cycle: i32,     /* cycle count */
    vix: usize,     /* index into vec variables */
    xvec: Vec<i32>, /* x register value vec */
    xsum: i32,
    svec: Vec<i32>, /* signal strength vec */
    ssum: i32,
    cvec: Vec<i32>  /* cycle vec at which to record xvec/svec */
}

impl CPU {
    pub fn new(mut cvec_arg: Vec<i32>) -> CPU {
        // let mut cvecdeq: VecDeque<i32> = VecDeque::from(cvec_arg);
        // cvecdeq.make_contiguous().sort();
        cvec_arg.sort(); /* sort incoming vec */
        CPU{
            x: 1,
            cycle: 0,
            vix: 0,
            xvec: vec![],
            xsum: 0,
            svec: vec![],
            ssum: 0,
            cvec: cvec_arg
        }
    }
    fn cycle_incr(&mut self, n: i32) {
        for _ix in 0..n {
            self.cycle += 1;
            if self.vix < self.cvec.len() {
                if self.cycle == self.cvec[self.vix] {
                    self.xsum += self.x;
                    self.ssum += self.x * self.cycle;
                    self.xvec.push(self.x);
                    self.svec.push(self.x * self.cycle);
                    self.vix += 1;
                }
            }
        }
    }
    pub fn addx(&mut self, v: i32) {
        self.cycle_incr(2);
        self.x += v;
        //println!("--> @ cycle {:3}, x = {:4}, s = {:6}",
    }
    pub fn noop(&mut self) { self.cycle_incr(1); }
}

fn parse_input(fname: &str, mut cvec: Vec<i32>) -> CPU {
    let addx_repat = Regex::new(r"^addx (-?\d+)").unwrap();
    let noop_repat = Regex::new(r"^noop").unwrap();
    let mut cpu = CPU::new(cvec);

    // println!("Parsing {fname}");
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ok_line) = line {
                // println!("{ok_line}");
                if let Some(caps) = addx_repat.captures(&ok_line) {
                    let vstr = caps.get(1).unwrap().as_str();
                    let v: i32 = vstr.parse::<i32>().unwrap();
                    // println!("addx, v = {v}");
                    cpu.addx(v);
                }
                else if noop_repat.is_match(&ok_line) {
                    // println!("noop!");
                    cpu.noop();
                }
            }
        }
    }

    for _ix in 0..cpu.xvec.len() {
        println!("@ cycle {:3}, x = {:4}, s = {:6}",
            cpu.cvec[_ix], 
            cpu.xvec[_ix], 
            cpu.svec[_ix]
        );
    }

    return cpu;
}