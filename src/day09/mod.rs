use std::collections::HashSet;
use std::fmt;
use std::cmp::max;
use regex::Regex;
use crate::utils::read_lines;
// use ndarray::prelude::*;
// use ndarray_stats::QuantileExt;


pub fn run(fname: &str) {
    part1(&fname);
    part2(&fname);
}

fn part1(fname: &str) {
    let r2 = input_parse(fname, 2);
    println!("part1: r2.tset.len() = {}", r2.tset.len());
}

fn part2(fname: &str) {
    let r10 = input_parse(fname, 10);
    // for itm in &r10.tset { println!("{itm}"); }
    println!("part2: r10.tset.len() = {}", r10.tset.len());
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct XY { x: i32, y: i32}

impl fmt::Display for XY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:3},{:3}]", self.x, self.y)
    }
}

impl XY{
    // self (head) moves arg
    pub fn add(&mut self, arg: &XY) {
        self.x += arg.x;
        self.y += arg.y;
    }

    // self (tail) chases arg
    pub fn chase(&mut self, arg: &XY) {
        let dx = arg.x - self.x;
        let dy = arg.y - self.y;
        // println!("        dx = {dx}, dy = {dy}");
        assert!(max(dx.abs(), dy.abs()) <= 2);
        // TODO - assert dx and dy are in expected ranges
        // doing this programatically is annoying, so just do the stupid
        // simple thing and match cases...
        let addarg = match (dx, dy) {
            // TODO - make this a simple HashMap ?
            // simple cardinal moves
            ( 2,  0) => XY{ x:  1, y:  0},
            (-2,  0) => XY{ x: -1, y:  0},
            ( 0,  2) => XY{ x:  0, y:  1},
            ( 0, -2) => XY{ x:  0, y: -1},
            // diagonal moves from cardinal moves
            ( 2,  1) => XY{ x:  1, y:  1},
            ( 2, -1) => XY{ x:  1, y: -1},
            (-2,  1) => XY{ x: -1, y:  1},
            (-2, -1) => XY{ x: -1, y: -1},
            ( 1,  2) => XY{ x:  1, y:  1},
            (-1,  2) => XY{ x: -1, y:  1},
            ( 1, -2) => XY{ x:  1, y: -1},
            (-1, -2) => XY{ x: -1, y: -1},
            // diagonal moves from diagonal moves
            ( 2,  2) => XY{ x:  1, y:  1},
            ( 2, -2) => XY{ x:  1, y: -1},
            (-2,  2) => XY{ x: -1, y:  1},
            (-2, -2) => XY{ x: -1, y: -1},
            // no move for everything else...
                _    => XY{ x:  0, y:  0},
        };
        self.add(&addarg);
    }
}

struct Rope { r: Vec<XY>, tset: HashSet<XY> }

impl Rope {
    pub fn new(rlen: usize) -> Rope {
        let mut rvec: Vec<XY> = vec![];
        for _ix in 0..rlen {
            rvec.push(XY{ x:0, y:0 });
        }
        return Rope { r: rvec, tset: HashSet::new() }
    }
    fn hmv(&mut self, dir: &str, n: i32) {
        // handle moving h
        let mvadd = match dir {
            "L" => XY{ x: -1, y:  0},
            "R" => XY{ x:  1, y:  0},
            "U" => XY{ x:  0, y:  1},
            "D" => XY{ x:  0, y: -1},
             &_ => XY{ x:  0, y:  0} /* report error? */
        };

        // println!("DBG: dir = {dir}, n={n}");
        for _nix in 0..n {
            // println!("  {} of {}", _nix+1, n);
            self.r[0].add(&mvadd);
            // println!("     0: {}", self.r[0]);
            for rix in 1..self.r.len() {
                let tmparg = self.r[rix-1].clone();
                self.r[rix].chase(&tmparg);
                // println!("    {rix:2}: {}", self.r[rix]);
            }
            self.tset.insert(self.r[self.r.len()-1].clone());
        }
    }
}

fn input_parse(fname: &str, ropelen: usize) -> Rope {
    // let mut out: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut rope = Rope::new(ropelen);

    // let mut out: HashSet<(i32, i32)> = HashSet::from([h]);
    let repat = Regex::new(r"^([LRUD]) (\d+)").unwrap();

    let mut loopix = 0;
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ok_line) = line {
                // println!("{:?}", ok_line);
                if let Some(caps) = repat.captures(&ok_line) {
                    let dir= caps.get(1).unwrap().as_str();
                    let nstr = caps.get(2).unwrap().as_str();
                    let n: i32 = nstr.parse::<i32>().unwrap();
                    // println!("DBG: dir={dir}, nstr={nstr}, n={n}");
                    rope.hmv(dir, n);
                }
            }
            loopix += 1;
            // if loopix > 50 { break; }
        }
    }
    return rope;
}
       
