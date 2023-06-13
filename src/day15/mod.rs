use std::fmt::format;
use std::{fs, str, num::ParseIntError, time::Instant};
use std::cmp::Ordering;
use regex::Regex;
// use std::collections::HashSet;

pub fn run(fname: &str) {
    /* TODO - make timing function? */
    let start = Instant::now();
    parse_file(fname);
    println!("parse_file took {:?} seconds", start.elapsed());
}    

#[derive(Debug, Clone, Copy, PartialEq)]
struct XY { x: i32, y: i32 }

impl str::FromStr for XY {
    type Err = ParseIntError; // ParseXYError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy_re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
        let caps= xy_re.captures(s).unwrap();
        return Ok(
            XY {
                x: caps.get(1).unwrap().as_str().parse::<i32>()?,
                y: caps.get(2).unwrap().as_str().parse::<i32>()?,
            }
        );
    }
}


fn parse_file(fname: &str) {
    let fstr = fs::read_to_string(fname)
        .expect(format!("File '{fname}' not readable.").as_str());

    let lines: Vec<&str> = fstr.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    /* parse each line, looking for sensor and beacon */
    let xy_pat = r"(x=-?\d+, y=-?\d+)";
    let line_pat = format!("Sensor at {xy_pat}: closest beacon is at {xy_pat}");
    let line_re = Regex::new(line_pat.as_str()).unwrap();
    println!("DBG: line_re = -->{line_re:?}<--");
    for (ix, line) in lines.iter().enumerate() {
        println!("How to parse line = -->{line}<--");
        if let Some(caps) = line_re.captures(line) {
            let s: XY = caps.get(1).unwrap().as_str().parse().unwrap();
            let b: XY = caps.get(2).unwrap().as_str().parse().unwrap();
            println!("{ix:2} : sensor = {s:?}, beacon = {b:?}");
        }
    }
}

