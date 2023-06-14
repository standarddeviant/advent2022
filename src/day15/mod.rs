use std::hash::Hash;
use std::{cmp::Ordering, fs, str, num::ParseIntError, time::Instant, iter::zip};
use std::collections::HashSet;
use regex::Regex;
// use std::collections::HashSet;

pub fn run(fname: &str, yrow: i32) {
    part1(fname, yrow);
}

fn part1(fname: &str, yrow: i32) {
    /* TODO - make timing function? */
    let start = Instant::now();
    let sbvec= parse_file(fname);
    println!("parse_file took {:?} seconds", start.elapsed());

    /* aggregate each source-beacon-coverage intersections w/ yrow */
    let mut intrs: Vec<(i32, i32)> = vec![];
    for (ix, sb) in sbvec.iter().enumerate() {
        if let Some(intr) = sb.row_intersection(yrow) {
            intrs.push(intr);
        }
    }

    /* combine overlapping intrs */
    let intrs = combine_intrs(intrs);
    let mut intrs_accum: i32 = intrs.iter()
        .map(|a: &(i32, i32)| a.1 - a.0 + 1)
        .sum();
    // println!("intrs_accum = {intrs_accum}");
    // for (ix, intr) in intrs.iter().enumerate() { 
    //     println!("combined intr [{ix:2}] = {intr:?}")
    // }

    /* subtract sources and beacons that are in yrow */
    let sb_intrs_overlap = intrs_contains(yrow, &intrs, &sbvec);
    intrs_accum -= sb_intrs_overlap;

    println!("part1: yrow={yrow}, intrs_accum = {intrs_accum}");

}    


fn intrs_contains(y: i32, intrs: &Vec<(i32, i32)>, sbvec: &Vec<SB>) -> i32 {
    let mut contained: HashSet<XY> = HashSet::new();
    for sb in sbvec {
        for xy in [sb.s, sb.b] {
            if xy.y != y { continue; }
            for intr in intrs {
                if intr.0 <= xy.x && xy.x <= intr.1 {
                    contained.insert(xy);
                    break;
                }
            }
        }
    }
    return contained.len() as i32;
    // return -1;
}

fn sort_intrs(intrs: &mut Vec<(i32, i32)>) {
    intrs.sort_by(|a, b| -> Ordering {
        /* key off lower first */
        if a.0 < b.0 { return Ordering::Less    }
        if a.0 > b.0 { return Ordering::Greater }
        /* key off higher second */
        if a.1 < b.1 { return Ordering::Less    }
        if a.1 > b.1 { return Ordering::Greater }
        /* return same if equal */
        return Ordering::Equal;
    });
}

fn combine_intrs(intrs_arg: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut intrs_out: Vec<(i32, i32)> = vec![];
    let mut intrs: Vec<(i32, i32)> = intrs_arg.clone();
    if intrs_arg.is_empty() { return vec![] }

    /* first, sort intrs */
    sort_intrs(&mut intrs);

    /* then combine into output */
    let mut i = intrs[0];
    for ix in 1..intrs.len() {
        let intr = intrs[ix];
        // println!("  DBG: i = {i:?}, intr = {intr:?}");
        /* check if overlapping */
        if i.1 >= intr.0 {
            if intr.1 > i.1 {
                // print!("    updating i from {i:?} ");
                i.1 = intr.1.max(i.1);
                // println!(" to {i:?} ");
            }
            continue;
        }
        intrs_out.push(i);
        i = intr;
    }
    intrs_out.push(i);

    return intrs_out;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, Copy, PartialEq)]
struct SB { s: XY, b: XY }
impl str::FromStr for SB {
    type Err = ParseIntError; // ParseXYError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy_pat = r"(x=-?\d+, y=-?\d+)";
        let line_pat = format!("Sensor at {xy_pat}: closest beacon is at {xy_pat}");
        let line_re = Regex::new(line_pat.as_str()).unwrap();
        let caps= line_re.captures(s).unwrap();
        return Ok(
            SB {
                s: caps.get(1).unwrap().as_str().parse().unwrap(),
                b: caps.get(2).unwrap().as_str().parse().unwrap()
            }
        );
    }
}
impl SB {
    pub fn row_intersection(&self, y: i32) -> Option<(i32, i32)> {
        let r = self.radius();
        let ydiff = (y - self.s.y).abs();
        if ydiff <= r {
            let d = r - ydiff;
            return Some((self.s.x-d, self.s.x+d));
        }
        return None;
    }
    fn radius(&self) -> i32 {
        (self.s.x - self.b.x).abs() + 
        (self.s.y - self.b.y).abs()
    }
}

fn parse_file(fname: &str) -> Vec<SB> {
    let fstr = fs::read_to_string(fname)
        .expect(format!("File '{fname}' not readable.").as_str());

    let lines: Vec<&str> = fstr.split("\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut out: Vec<SB> = vec![];
    for (ix, line) in lines.iter().enumerate() {
        let sb_parse: Result<SB, _> = line.parse();
        if let Ok(sb) = sb_parse {
            // println!("ix = {ix}, sb = {sb:?}");
            out.push(sb);
        }
    }

    return out;
}

