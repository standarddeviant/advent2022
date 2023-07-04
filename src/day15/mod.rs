use std::fmt::format;
use std::hash::Hash;
use std::{cmp::{Ordering, min, max}, fs, ops, str};
use std::{num::ParseIntError, time::Instant, iter::zip};
use std::collections::HashSet;
use log::{debug, info};
use plotters::style::full_palette::PURPLE;
use regex::Regex;
use geo::{Coord, LineString, Polygon, Contains, polygon, BooleanOps, MultiPolygon, CoordsIter, Point, InteriorPoint, EuclideanDistance, EuclideanLength, Centroid};
use geo::Area;
// use geo_booleanop::boolean::BooleanOp;
use indicatif::ProgressBar;
// use termplot::*;
use plotters::prelude::*;

type XYNum = i32;

pub fn run(fname: &str, p1_yrow: i32, p2_bbox: (XYNum, XYNum)) {
    part1(fname, p1_yrow);
    part2(fname, p2_bbox);
}

fn multi_poly_draw(mp: &MultiPolygon, name: &str, bbarg: (XYNum, XYNum)) {
    let fname = format!("day15_{name}.svg");
    let drawing_area = 
        SVGBackend::new(fname.as_str(), (500, 500))
        .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let epts: Vec<Coord> = mp.exterior_coords_iter().collect();
    let minx = epts.iter()
        .map(|ept| ept.x)
        .reduce(|e1, e2| e1.min(e2))
        .unwrap();
    let maxx = epts.iter()
        .map(|ept| ept.x)
        .reduce(|e1, e2| e1.max(e2))
        .unwrap();
    let miny = epts.iter()
        .map(|ept| ept.y)
        .reduce(|e1, e2| e1.min(e2))
        .unwrap();
    let maxy = epts.iter()
        .map(|ept| ept.y)
        .reduce(|e1, e2| e1.max(e2))
        .unwrap();

    let x_spec = minx .. maxx;
    let y_spec = miny .. maxy;
    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder.margin(10).set_left_and_bottom_label_area_size(20);

    let mut chart_context = chart_builder
        .margin_bottom(30)
        .build_cartesian_2d(x_spec, y_spec)
        .unwrap();
        // .build_cartesian_3d(0.0..4.0, 0.0..3.0, 0.0..2.7).unwrap();
    chart_context.configure_mesh().draw().unwrap();

    for pix in 0..mp.0.len() {
        let p = mp.0[pix].clone();
        let epts = p.exterior().clone().into_points();
        chart_context.draw_series(
            LineSeries::new(
                //epts.iter().map(),
                epts.iter().map(
                    |ept| (ept.x(), ept.y())
                ),
                BLUE
            ).point_size(2)
        )
        .unwrap();
        let ints= p.interiors();
        for intls in ints {
            chart_context.draw_series(
                LineSeries::new(
                    //epts.iter().map(),
                    intls.clone().into_iter().map(
                        |ipt| (ipt.x, ipt.y)
                    ),
                    RED 
                ).point_size(2)
            )
            .unwrap();
        }
    }
    chart_context.draw_series(
        LineSeries::new(
            [
                (bbarg.0 as f64, bbarg.0 as f64),
                (bbarg.0 as f64, bbarg.1 as f64),
                (bbarg.1 as f64, bbarg.1 as f64),
                (bbarg.1 as f64, bbarg.0 as f64),
                (bbarg.0 as f64, bbarg.0 as f64)
            ],
            PURPLE 
        ).point_size(5)
    )
    .unwrap();
}


fn poly_print(p: &Polygon, name: &str) {
    println!("{name} = [");
    for (ix, c) in p.exterior().into_iter().enumerate() {
        // println!("    [{ix:2}] : {}, {}", c.x, c.y);
        println!("  ({}, {}),", c.x, c.y);
    }
    println!("]");
}

fn multi_poly_print(mp: &MultiPolygon, name: &str) {
    println!("{name} = [");
    for p in mp.0.iter() {
        println!("    [");
        for (ix, c) in p.exterior().into_iter().enumerate() {
            // println!("    [{ix:2}] : {}, {}", c.x, c.y);
            println!("      ({}, {}),", c.x, c.y);
        }
        println!("    ],");
    }
    println!("]");
}

fn poly_union(p1: &MultiPolygon, p2: &MultiPolygon) -> MultiPolygon {
    let tmp = p1.union(&p2);
    /*
    for tmptmp in tmp.0 {
        println!("tmp poly = {tmptmp:?}");
    }
    */
    return p1.union(&p2);
}

/*
fn poly_test() {
    let p1 = polygon![ (x: 0., y: 0.), (x:10., y: 0.), (x:10., y:10.), (x: 0., y:10.) ];
    let p2 = polygon![ (x:-5., y:-5.), (x: 5., y:-5.), (x: 5., y: 5.), (x:-5., y: 5.) ];
    let p3 = poly_union(&p1, &p2);
    let p4 = //MultiPolygon::from(
        polygon![ (x: -2., y: -10.), (x: 2., y: -10.), (x: 2., y: -1.), (x: -2., y: -1.)];
    let p5 = poly_union(&p3, &p4);
    poly_print(&p1, "p1");
    poly_print(&p2, "p2");
    poly_print(&p3, "p3");
    poly_print(&p4, "p4");
    poly_print(&p5, "p5");
} */
 


fn part2(fname: &str, bbarg: (XYNum, XYNum)) {
    /* TODO - make timing function? */
    let start = Instant::now();
    let sbvec = parse_file(fname);

    info!("sbvec.len() = {}", sbvec.len());
    for (ix, sb) in sbvec.iter().enumerate() {
        debug!("sb [{ix}] = {sb:?}");
    }

    let mut poly_vec: Vec<Polygon> = vec![];
    let mut poly_agg: MultiPolygon<f64> = MultiPolygon::new(vec![]);
    for (sbix, sb) in sbvec.iter().enumerate() {
        let sb_poly = sb.multi_poly();
        poly_vec.push(sb.poly());
        // println!("sbix = {sbix}");
        // poly_print(&sb_poly.0[0], "sb_poly");
        let tmp_poly = poly_union(&poly_agg, &sb_poly);
        poly_agg = tmp_poly;
        let tmps = format!("poly_agg_partial_{sbix:?}");
        multi_poly_draw(&poly_agg, tmps.as_str(), bbarg);
    }
    let multi_poly_vec: MultiPolygon<f64> = MultiPolygon::from(poly_vec);
    // multi_poly_print(&poly_agg, "poly_agg");
    // multi_poly_draw(&poly_agg, "poly_agg", bbarg);
    // multi_poly_draw(&multi_poly_vec, "multi_poly_vec", bbarg);

    let mut theone: Coord = Coord{x: -1., y: -1.};
    for p in poly_agg.0 {
        for int in p.interiors() {
            let tmpp = Polygon::new(int.clone(), vec![]);
            let minside = int.lines().into_iter()
                .map(|tmpl| tmpl.euclidean_length())
                .reduce(|l1, l2| l1.min(l2)).unwrap();
            let maxside = int.lines().into_iter()
                .map(|tmpl| tmpl.euclidean_length())
                .reduce(|l1, l2| l1.max(l2)).unwrap();
            if (maxside - minside).abs() > 0.001 {
                continue;
            }

            info!("yippee!!!!! we got it!");
            info!("interior @ {int:?}");
            info!("    unsigned_area = {}", tmpp.unsigned_area());
            info!("    minside       = {minside}");
            info!("    maxside       = {maxside}");
            info!("    centroid      = {:?}", tmpp.centroid());
            theone = tmpp.centroid().unwrap().0;
        }
    }
    let freq: i128 = (theone.x as i128) * (4000000 as i128) + (theone.y as i128);
    println!("part2: distress beacon is at {theone:?}");
    println!("part2: tuning frequency is {freq}");
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

    println!("part1: yrow={yrow}, intrs_accum = {intrs_accum}\n");

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
        debug!(">>> INTERSECTION @ {p:?}, s0={s0:?}, s1={s1:?}");
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
    pub fn multi_poly(&self) -> MultiPolygon {
        let mut pts = self.points().map(|xy| { (xy.x as f64, xy.y as f64)});
        let line_string: LineString = LineString::from_iter( pts.into_iter() );
        let poly = Polygon::new(line_string, vec![]);
        let out = MultiPolygon::from(vec![poly]);
        return out;
    }
    pub fn poly_contains(&self, xy: &XY) -> bool {
        let p = self.poly();
        let c: Coord<f64> = Coord { x: xy.x as f64, y: xy.y as f64 };
        return p.contains(&c);
    }
    pub fn poly(&self) -> Polygon {
        // let eps = 0.001_f64;
        let pts = self.points().map(|xy| { (xy.x as f64, xy.y as f64)});
        /* make diamond slightly bigger in prep for call to 'contains'
           NOTE: this is just to catch points that lie along the edge of a diamond */
        // pts[0].1 -= eps; // N, y
        // pts[1].0 += eps; // E, x
        // pts[2].1 += eps; // S, y
        // pts[3].0 -= eps; // W, x
        
        let line_string: LineString = LineString::from_iter( pts.into_iter() );
        return Polygon::new(line_string, vec![]);
    }
    // https://math.stackexchange.com/a/1425630/97198
    /*
    pub fn poly_intersections(&self, other: &SB) -> Vec<(f64, f64)> {
        let mut out: Vec<(f64, f64)> = vec![];
        // fn L(A: XY, B: XY, t: T) -> XY {
        //     return XY{
        //         x: (B.x - A.x)*t + A.x,
        //         y: (B.y - A.y)*t + A.y
        //     }
        // }

        // println!("\n");
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
    } */
    fn points(&self) -> [XY; 4] {
        let r = self.radius();
        debug!("r = {r}");
        /* return ordered points as NESW (clockwise) */
        let out = [
            XY{ x: self.s.x    , y: self.s.y - r}, // N
            XY{ x: self.s.x + r, y: self.s.y    }, // E
            XY{ x: self.s.x    , y: self.s.y + r}, // S
            XY{ x: self.s.x - r, y: self.s.y    }, // W
        ];
        debug!("self = {self:?}, points = {out:?}");
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

