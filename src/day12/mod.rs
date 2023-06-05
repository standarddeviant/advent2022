use std::iter::Enumerate;
use std::{cmp, fmt, fs, io};
// use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use pathfinding::directed::dijkstra;
use pathfinding::prelude::dijkstra;
use ndarray::{s, ArcArray2, Ix2, Array, stack, ArrayViewMut, ArcArray};
use pathfinding::prelude::directions::W;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Part {P1, P2}

// #[derive(Debug, PartialEq, Eq)]
// struct ParseMapError;

#[derive(Debug)]
struct YXV{x: usize, y: usize, v: i32}
 
#[derive(Clone, Eq, Hash)]
struct Pos { 
    // TODO - use ndarray or peroxide ???
    p: Part,
    map: ArcArray2<i32>,
    x: usize,
    y: usize,
    v: i32
}

impl fmt::Display for Pos {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} [{},{}]", self.v, self.x, self.y)
    }
}
impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{},{}]", self.v, self.x, self.y)
    }
}
 

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        return (self.x == other.x) && (self.y == other.y);
    }
}


impl Pos {
    pub fn from_misc(p: Part, map: ArcArray2<i32>, yxv: YXV) -> Pos {
        Pos{
            p: p,
            map: map.clone(),
            y: yxv.y,
            x: yxv.x,
            v: yxv.v
        }

    }
    // satisfy successors for pathfinding's djikstra algo
    fn successors(&self) -> Vec<(Pos, usize)> {
        let mut out: Vec<(Pos, usize)> = vec![];
        // let map = self.map.get(index)
        // let (nrows, ncols) = ( map.len(), map[0].len() );
        // look in four directions to see if it's a viable path

        let (nrows, ncols) = (self.map.nrows(), self.map.ncols());
        let mut neighbors: Vec<Ix2> = vec![];
        if self.y >    0    { neighbors.push(Ix2(self.y-1, self.x  )) }
        if self.y < nrows-1 { neighbors.push(Ix2(self.y+1, self.x  )) }
        if self.x >    0    { neighbors.push(Ix2(self.y  , self.x-1)) }
        if self.x < ncols-1 { neighbors.push(Ix2(self.y  , self.x+1)) }
        // println!("neighbors = {neighbors:?}");

        /*
        let neighbors = [
            Ix2( self.y+1 , self.x   ), /* N */
            Ix2( self.y   , self.x+1 ),  /* E */
            Ix2( self.y   , self.x-1 ), /* W */
            Ix2( self.y-1 , self.x   ), /* S */
        ];
        */
        for ix in neighbors {
            // println!("ix = {ix:?}");
            if let Some(okv) = self.map.get(ix) {
                // println!("*okv = {}, self.v + 1 = {}", *okv, self.v+1);
                let chk = match self.p {
                    Part::P1 => *okv <= self.v + 1,
                    Part::P2 => *okv >= self.v - 1  /* reverse... */
                };
                if chk {
                    let p = Pos {
                        p: self.p,
                        map: self.map.clone(),
                        y: ix[0],
                        x: ix[1],
                        v: *okv
                    };
                    out.push((p, 1));
                }
            }
        }
        // println!("@ {}, successors = ", self);
        // for ix in 0..out.len() {
        //      println!("    {ix}: {} -> {}", self, out[ix].0);
        // }
        return out;
    } /* end: fn successors(&self) -> Vec<(Pos<i32>, usize)> */
}


/* TODO - make this a FromStr impl */
fn parse_input(p12: Part, s: &str) -> (ArcArray2<i32>, Pos, Pos, Vec<Pos>) {
    // type Err = ParseMapError;
    let lines: Vec<&str> = s.split("\n")
        .map(|L| L.trim())
        .filter(|L| !L.is_empty())
        .collect();
    let (nrows, ncols) = (lines.len(), lines[0].len());
    let mut a: Array<i32, Ix2> = Array::zeros((nrows, ncols));
    let mut rix: usize =0; 
    let mut start: YXV = YXV{ x: 0, y: 0, v: -1};
    let mut end  : YXV = YXV{ x: 0, y: 0, v: -1};
    let mut zlist_yxv: Vec<YXV> = vec![];
    let mut zlist_pos: Vec<Pos> = vec![];
    println!("     0123456789");
    for line in lines {
        if line.is_empty() {
            continue;
        }
        println!("{rix:2} | {line}");
        let mut cix: usize = 0;
        for c in line.chars() {
            let v: i32 = {
                if 'a' <= c && c <= 'z' {  c  as i32 - 'a' as i32 }
                else if c == 'S'        { 'a' as i32 - 'a' as i32 }
                else if c == 'E'        { 'z' as i32 - 'a' as i32 }
                else                    {  -1 as i32              }
            };
            if v == 0 {
                zlist_yxv.push(YXV{y: rix, x: cix, v: v});
            }

            a[[rix, cix]] = v;
            if c == 'S' {
                start = YXV{y: rix, x: cix, v: v};
            }
            else if c == 'E' {
                end   = YXV{y: rix, x: cix, v: v};
            }
            cix += 1;
        }
        // println!("row = {row1x:?}");
        // rows.append(axis, array)
        rix += 1;
    }

    println!("!!!!!! START @ {start:?}");
    println!("!!!!!! END   @ {end:?}");

    // let a = Array2::from([]);
    let map: ArcArray2<i32> = ArcArray2::from( a );
    for yxv in zlist_yxv {
        zlist_pos.push(Pos::from_misc(p12, map.clone(), yxv));
    }
    // let alist: Vec<Pos> = zlist.iter().map(
    (
        map.clone(),
        Pos{p: p12, map: map.clone(), x: start.x, y: start.y, v: start.v}, // start
        Pos{p: p12, map: map.clone(), x:   end.x, y:   end.y, v:   end.v}, // end 
        zlist_pos
    )
}



fn dijkstra_test() {
    static GOAL: (i32, i32) = (4, 6);
    let result = 
    dijkstra(&(1, 1),
        |&(x, y)| vec![(x+1,y+2), (x+1,y-2), (x-1,y+2), (x-1,y-2),
        (x+2,y+1), (x+2,y-1), (x-2,y+1), (x-2,y-1)]
        .into_iter().map(|p| (p, 1)),
        |&p| p == GOAL
    );
    println!("result = {result:?}");
    assert_eq!(result.expect("no path found").1, 4);
}

pub fn run(fname: &str) {
    println!("yolo!");
    // dijkstra_test();
    // return;
    // part1
    let file = 
        fs::read_to_string(fname)
        .expect(format!("File '{fname}' not readable.").as_str());

    let (map, start, end, zlist) = 
        parse_input(Part::P1, file.as_str());
    // println!("start = {start:?}");
    // println!("end   = {end:?}");
    // println!("map   = \n{map}");
    // println!("zlist =");
    // for z in &zlist {
    //     println!("z = {z}");
    // }

    let p1_result = dijkstra(
        &start, 
        |p| p.successors(),
        |p| *p == end
    );

    // println!("start = {start:?}");
    // println!("end   = {end:?}");
    // println!("result = {:?}", result);

    let (map, start, end, zlist) = 
        parse_input(Part::P2, file.as_str());
    let p2_result = dijkstra(
        &end, 
        |p| p.successors(),
        |p| zlist.contains(p)
    );

    if let Some(p1) = p1_result {
        println!("part1: solution found in {} steps", p1.1)
    }

    if let Some(p2) = p2_result {
        println!("part2: solution found in {} steps", p2.1)
    }



    // println!("map[s![16:24, 0..6]] = {:?}", map.slice(s![16..24, 0..6]));
    // assert_eq!(result.expect("no path found").1, 4);
}
