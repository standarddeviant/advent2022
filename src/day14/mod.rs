use std::{fs, str, num::ParseIntError, time::Instant};
use std::cmp::Ordering;

use regex::internal::Inst;
// use std::collections::HashSet;

pub fn run(fname: &str) {
    let start = Instant::now();
    part1(fname);
    println!("part1 took {:?} seconds", start.elapsed());
}    

fn part1(fname: &str) {
    let mut f = parse_file(fname);
    loop {
        match f.place_sand() {
            Some(xy) => {},
            None         => {
                break;
            }
        }
    }
    println!("part1: placed_sand count = {}", f.count);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct XY { x: i32, y: i32 }

impl str::FromStr for XY {
    type Err = ParseIntError; // ParseXYError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let xy: Vec<&str> = s.split(",").collect();
        return Ok(
            XY {
                x: xy[0].parse::<i32>()?,
                y: xy[1].parse::<i32>()?
            }
        );
    }
}


/*
impl cmp::PartialOrd for XY {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if self.y > other.y 
        return None;
    }
}
*/

fn fill_endpoints(ep: Vec<XY>) -> Vec<XY> {
    let mut fill: Vec<XY> = vec![];
    for ix in 1..ep.len() {
        let (dst, src) = (ep[ix], ep[ix-1]);
        let mut p = src;
        let d = {
            if      dst.x < src.x { XY{x: -1, y:  0} }
            else if dst.x > src.x { XY{x:  1, y:  0} }
            else if dst.y < src.y { XY{x:  0, y: -1} }
            else if dst.y > src.y { XY{x:  0, y:  1} }
            else { continue; }
        };
        // println!("ix={ix}, src={src:?}, dst={dst:?}, d={d:?}");
        while p != dst {
            fill.push(p);
            p.x += d.x;
            p.y += d.y;
        }
    }
    if ep.len() > 0 { fill.push(ep[ep.len()-1]); }
    return fill;
}

fn parse_line_fill(line: &str) -> Vec<XY> {
    let mut endpoints: Vec<XY> = vec![];
    let xy_strs: Vec<&str> = line.split(" -> ").collect();
    for xy_str in xy_strs {
        let xy = xy_str.parse::<XY>();
        if let Ok(xy) = xy {
            // println!("Ok(xy) = {xy:?}");
            endpoints.push(xy);
        }
    }
    // println!("endpoints = {endpoints:?}");
    let fill = fill_endpoints(endpoints);
    // println!("fill = {fill:?}");
    return fill;
}

// #[derive(Debug, Clone, Copy)]
// enum FillType {Rock, Sand}

struct Formation {
    fill: Vec<XY>,
    count: i32,
    yvoid: i32
}

impl Formation {
    fn place_sand(&mut self) -> Option<XY> {
        /* send location */
        let mut s = XY{ x: 500, y: 0 };
        loop {
            /* return None if this sand has fallen into the void */
            if s.y >= self.yvoid { return None; }

            /* look for free space in order of dn, dnleft, dnright   */
            // println!("DBG: s = {s:?}, yvoid={}", self.yvoid);
            let space_dn = !self.fill.contains( &XY{x: s.x, y: s.y+1} );
            if space_dn { s.y += 1; continue; }
            let space_dnleft= !self.fill.contains( &XY{x: s.x-1, y: s.y+1} );
            if space_dnleft{ s.x -= 1; s.y += 1; continue; }
            let space_dnright= !self.fill.contains( &XY{x: s.x+1, y: s.y+1} );
            if space_dnright { s.x += 1; s.y += 1; continue; }

            /* if we get here, the sand gets placed in fill, or falls to void */
            // println!("placed sand @ {s:?}");
            self.fill.push(s);
            self.count += 1;
            return Some(s);
        }
    }
}

impl str::FromStr for Formation {
    type Err = ParseIntError; // ParseXYError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split("\n")
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let mut fill: Vec<XY> = vec![];
        for line in lines {
            fill.extend(parse_line_fill(line));
        }

        fill.sort_by(
            |a: &XY, b: &XY| -> Ordering {
                if      a.y > b.y { Ordering::Greater }
                else if a.y < b.y { Ordering::Less    }
                else              { Ordering::Equal   }
            }
        );
        let yvoid: i32 = fill[fill.len() - 1].y;

        return Ok(Formation { fill: fill, count: 0, yvoid: yvoid });
    }
}



fn parse_file(fname: &str) -> Formation {
    let mut out: Vec<XY> = vec![];
    // let mut out: Vec<Vec<Value>> = vec![];
    let fstr = fs::read_to_string(fname)
        .expect(format!("File '{fname}' not readable.").as_str());
    return fstr.parse().unwrap();
}
