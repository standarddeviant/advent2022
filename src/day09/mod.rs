use std::collections::HashSet;
use std::fmt;
use regex::Regex;
use crate::utils::read_lines;
// use ndarray::prelude::*;
// use ndarray_stats::QuantileExt;


pub fn run(fname: &str) {
    let s = input_parse(fname);
    part1(&s);
}

fn part1(s: &Day09State) {
    println!("part1: s.tset.len() = {}", s.tset.len());
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
            // diagonal moves
            ( 2,  1) => XY{ x:  1, y:  1},
            ( 2, -1) => XY{ x:  1, y: -1},
            (-2,  1) => XY{ x: -1, y:  1},
            (-2, -1) => XY{ x: -1, y: -1},
            ( 1,  2) => XY{ x:  1, y:  1},
            (-1,  2) => XY{ x: -1, y:  1},
            ( 1, -2) => XY{ x:  1, y: -1},
            (-1, -2) => XY{ x: -1, y: -1},
            // no move for everything else...
                  _  => XY{ x:  0, y:  0},
        };
        self.add(&addarg);
    }
}

struct Day09State { h: XY, t: XY, tset: HashSet<XY> }

impl Day09State {
    pub fn new() -> Day09State {
        Day09State {
            h: XY{ x:0, y:0 },
            t: XY{ x:0, y:0 },
            tset: HashSet::new()
        }
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
        for _ix in 0..n {
            // print!("   head moves {:}\n", self.h);
            self.h.add(&mvadd);
            // print!("              {:}\n", self.h);
            // print!("   tail moves {:}\n", self.t);
            self.t.chase(&self.h);
            // print!("              {:}\n", self.t);
            self.tset.insert(self.t.clone());
        }
    }
}

fn input_parse(fname: &str) -> Day09State {
    // let mut out: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);
    let mut s = Day09State::new();

    // let mut out: HashSet<(i32, i32)> = HashSet::from([h]);
    let repat = Regex::new(r"^([LRUD]) (\d+)").unwrap();

    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ok_line) = line {
                // println!("{:?}", ok_line);
                if let Some(caps) = repat.captures(&ok_line) {
                    let dir= caps.get(1).unwrap().as_str();
                    let nstr = caps.get(2).unwrap().as_str();
                    let n: i32 = nstr.parse::<i32>().unwrap();
                    // println!("DBG: dir={dir}, nstr={nstr}, n={n}");
                    s.hmv(dir, n);
                }
            }
       }
    }

    return s;
}
       
