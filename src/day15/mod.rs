use std::hash::Hash;
use std::{cmp::{Ordering, min, max}, fs, ops, str};
use std::{num::ParseIntError, time::Instant, iter::zip};
use std::collections::HashSet;
use log::{debug, info};
use regex::Regex;
// use std::collections::HashSet;

type XYNum = i32;

pub fn run(fname: &str, p1_yrow: i32, p2_bbox: (XYNum, XYNum)) {

    part1(fname, p1_yrow);
    part2(fname, p2_bbox);
}


fn part2(fname: &str, bbarg: (XYNum, XYNum)) {
    /* TODO - make timing function? */
    let start = Instant::now();
    let sbvec = parse_file(fname);

    for (ix, sb) in sbvec.iter().enumerate() {
        debug!("sb [{ix}] = {sb:?}");
    }

    for ix in 1..sbvec.len() {
        let istmp = sbvec[0].poly_intersections(&sbvec[ix]);
    }

}

fn part1(fname: &str, yrow: i32) {
    /* TODO - make timing function? */
    let start = Instant::now();
    let sbvec= parse_file(fname);
    debug!("parse_file took {:?} seconds", start.elapsed());

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
        .map(|a: &(XYNum, XYNum)| a.1 as i32 - a.0 + 1)
        .sum();
    // info!("intrs_accum = {intrs_accum}");
    // for (ix, intr) in intrs.iter().enumerate() { 
    //     info!("combined intr [{ix:2}] = {intr:?}")
    // }

    /* subtract sources and beacons that are in yrow */
    let sb_intrs_overlap = intrs_contains(yrow, &intrs, &sbvec);
    intrs_accum -= sb_intrs_overlap;

    info!("part1: yrow={yrow}, intrs_accum = {intrs_accum}\n");

}    


fn intrs_contains(y: XYNum, intrs: &Vec<(XYNum, XYNum)>, sbvec: &Vec<SB>) -> i32 {
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

fn sort_intrs(intrs: &mut Vec<(XYNum, XYNum)>) {
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

fn combine_intrs(intrs_arg: Vec<(XYNum, XYNum)>) -> Vec<(XYNum, XYNum)> {
    let mut intrs_out: Vec<(XYNum, XYNum)> = vec![];
    let mut intrs: Vec<(XYNum, XYNum)> = intrs_arg.clone();
    if intrs_arg.is_empty() { return vec![] }

    /* first, sort intrs */
    sort_intrs(&mut intrs);

    /* then combine into output */
    let mut i = intrs[0];
    for ix in 1..intrs.len() {
        let intr = intrs[ix];
        // info!("  DBG: i = {i:?}, intr = {intr:?}");
        /* check if overlapping */
        if i.1 >= intr.0 {
            if intr.1 > i.1 {
                // print!("    updating i from {i:?} ");
                i.1 = intr.1.max(i.1);
                // info!(" to {i:?} ");
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
struct XY { x: XYNum, y: XYNum}

impl XY {
    pub fn new(x: XYNum, y: XYNum) -> Self {
        return XY {x: x, y: y}
    }
}

impl ops::Sub for XY {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl ops::Mul<i32> for XY {
    type Output = Self;
    fn mul(self, other: i32) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other
        }
    }
}
// fn x(u: XY, v: XY) -> f64 { (u.x * v.y - u.y * v.x) as f64 }
fn segment_intersection(s0: (XY, XY), s1: (XY, XY)) -> Vec<(f64, f64)> {
    let m0 = (s0.1.y-s0.0.y) as f64 / (s0.1.x-s0.0.x) as f64;
    let b0 = (s0.0.y - s0.0.x) as f64;

    let m1 = (s1.1.y-s1.0.y) as f64 / (s1.1.x-s1.0.x) as f64;
    let b1 = (s1.0.y - s1.0.x) as f64;

    let (a, c, b, d) = (m0, b0, m1, b1);

    let tmp = (d-c) / (a-b);
    let p = (tmp, a * tmp + c);

    let xrange = (
        min( min(s0.0.x, s0.1.x), min(s1.0.x, s1.1.x) ),
        max( max(s0.0.x, s0.1.x), max(s1.0.x, s1.1.x) )
    );

    // debug!("");
    debug!("s0={s0:?}, m0={m0}, b0={b0}");
    debug!("s1={s1:?}, m1={m1}, b1={b1}");
    debug!("    p = {p:?}");
    debug!("    xrange = {xrange:?}");
    if xrange.0 as f64 <= p.0 && p.0 <= xrange.1 as f64 {
        info!(">>> INTERSECTION @ {p:?}, s0={s0:?}, s1={s1:?}");
        return vec![p];
    }
    return vec![];
}

impl str::FromStr for XY {
    type Err = ParseIntError; // ParseXYError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy_re = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
        let caps= xy_re.captures(s).unwrap();
        return Ok(
            XY {
                x: caps.get(1).unwrap().as_str().parse::<XYNum>()?,
                y: caps.get(2).unwrap().as_str().parse::<XYNum>()?,
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
    // https://math.stackexchange.com/a/1425630/97198
    pub fn poly_intersections(&self, other: &SB) -> Vec<(f64, f64)> {
        let mut out: Vec<(f64, f64)> = vec![];
        // fn L(A: XY, B: XY, t: T) -> XY {
        //     return XY{
        //         x: (B.x - A.x)*t + A.x,
        //         y: (B.y - A.y)*t + A.y
        //     }
        // }

        println!("\n");
        let a = self.points();
        let b = other.points();

        /* check all four line segments against different-slope other line segments */
        out.extend(segment_intersection( (a[0], a[1]), (b[1], b[2]) ));
        out.extend(segment_intersection( (a[0], a[1]), (b[3], b[0]) ));
        out.extend(segment_intersection( (a[1], a[2]), (b[2], b[3]) ));
        out.extend(segment_intersection( (a[1], a[2]), (b[0], b[1]) ));
        out.extend(segment_intersection( (a[2], a[3]), (b[3], b[0]) ));
        out.extend(segment_intersection( (a[2], a[3]), (b[1], b[2]) ));
        out.extend(segment_intersection( (a[3], a[0]), (b[0], b[1]) ));
        out.extend(segment_intersection( (a[3], a[0]), (b[2], b[3]) ));

        return out;
    }
    fn points(&self) -> [XY; 4] {
        let r = self.radius();
        info!("r = {r}");
        /* return ordered points as NESW (clockwise) */
        let out = [
            XY{ x: self.s.x    , y: self.s.y - r}, // N
            XY{ x: self.s.x + r, y: self.s.y    }, // E
            XY{ x: self.s.x    , y: self.s.y + r}, // S
            XY{ x: self.s.x - r, y: self.s.y    }  // W
        ];
        info!("self = {self:?}, points = {out:?}");
        return out;
    }
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
            // info!("ix = {ix}, sb = {sb:?}");
            out.push(sb);
        }
    }

    return out;
}

