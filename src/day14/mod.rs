use std::{fs, str, num::ParseIntError, time::Instant};
use std::cmp::Ordering;
// use std::collections::HashSet;

pub fn run(fname: &str) {
    let start = Instant::now();
    part1(fname);
    println!("part1 took {:?} seconds", start.elapsed());

    // TODO - see if there is a "timeit" function in rust
    part2(fname);
}    

fn part1(fname: &str) {
    let mut f = parse_file(fname);
    loop {
        match f.place_sand(true) {
            Some(_xy) => {        },
            None          => { break; }
        }
    }
    println!("part1: placed_sand count = {}", f.count);
}

fn part2(fname: &str) {
    let mut f = parse_file(fname);
    let mut ix = 0;
    let start = Instant::now();
    loop {
        match f.place_sand(false) {
            Some(_xy) => {        },
            None          => { break; }
        }
        if ix % 1000 == 0 {
            println!("DBG: ix = {ix} @ {:?}", start.elapsed());
        }
        ix += 1;
    }
    println!("part2: placed_sand count = {}", f.count);
    println!("part2 took {:?} seconds", start.elapsed());
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
    path: Vec<XY>,
    fill: Vec<XY>,
    count: i32,
    yvoid: i32
}

impl Formation {
    fn place_sand(&mut self, p1: bool) -> Option<XY> {
        let p2 = !p1;
        /* send location */
        'outer: loop {
            /* check if the path is totally empty, i.e., that we have
               1. tried to place sand from all known path points
               2. failed to place sand at all those path points */
            if self.path.is_empty() { return None; }

            /* set current start pos from last point in path */
            let mut s = self.path[self.path.len() - 1].clone();

            /* if p1: return None if this sand has fallen into the void */
            // TODO - make p1 a struct var?
            if p1 && s.y >= self.yvoid { return None; }

            /*
            if p2 {
                if s.y == self.yvoid + 1 { 
                    if !self.fill.contains(&s) {
                        self.fill.push(s);
                    }
                    self.path.pop();
                    continue;
                }
            }
            */

            /* look for free space in order of dn, dnleft, dnright   */
            let spaces: Vec<XY> = vec![
                XY{x: s.x  , y: s.y+1}, // dn
                XY{x: s.x-1, y: s.y+1}, // dnleft
                XY{x: s.x+1, y: s.y+1}  // dnright
            ];
            // println!("\nspaces = {spaces:?}");
            let spaces: Vec<XY> = spaces.into_iter().filter(|a| a.y <= 1+self.yvoid).collect();
            // println!("spaces = {spaces:?}");
            // .iter().filter(|a| a.y <= self.yvoid).collect();

            /*
            if s.x == 500 && s.y ==  7 {
                return None;
            }
            */

            // println!("testing s = {s:?}");
            // println!("path.len() = {:?}", self.path.len());
            for space in &spaces {
                // println!("self.fill.contains({space:?}) = {:?}", self.fill.contains(&space));
                if !self.fill.contains(&space) {
                    // println!("PUSHING SPACE TO PATH: {space:?}");
                    self.path.push(*space);
                    continue 'outer;
                }
            }

            /* 
            if p2 {
                let all_filled = 
                    (&spaces).clone().iter()
                    .all(|a| self.fill.contains(a));
                if all_filled {
                    // println!("ALL FILLED @ {s:?}, yvoid={}", self.yvoid);
                    // println!("self.path.len() = {}", self.path.len());
                    // println!("self.path = {:?}", self.path);
                    self.path.pop();
                    self.fill.push(s);
                    self.count += 1;
                    // println!("self.path.len() = {}", self.path.len());
                    // println!("self.path = {:?}", self.path);
                    continue;
                }
            }
            */

            /* getting here means we can place sand @ s, b/c self.fill already
               contains dn, dnleft, and dnright */
            // println!("pushing {s:?}");
            self.fill.push(s); /* push XY to fill  */
            self.path.pop();   /* pop XY from path */
            self.count += 1;   /* increase count   */
            return Some(s);    /* return Some(XY)  */
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

        let start_xy = XY { x: 500, y: 0 };
        return Ok(
            Formation{
                path: vec![start_xy],
                fill: fill,
                count: 0,
                yvoid: yvoid
            }
        );
    }
}



fn parse_file(fname: &str) -> Formation {
    let fstr = fs::read_to_string(fname)
        .expect(format!("File '{fname}' not readable.").as_str());
    return fstr.parse().unwrap();
}
