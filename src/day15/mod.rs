use std::hash::Hash;
use std::{fs, ops, str};
use std::{num::ParseIntError, time::Instant};
use log::{debug, info};
use plotters::style::full_palette::PURPLE;
use regex::Regex;
use geo::{Coord, LineString, Polygon, polygon, BooleanOps, MultiPolygon, CoordsIter, EuclideanLength, Centroid};
use geo::Area;
// use geo_booleanop::boolean::BooleanOp;
use plotters::prelude::*;

type XYNum = i32;

pub fn run(fname: &str, p1_yrow: i32, p2_bbox: (XYNum, XYNum)) {
    part1(fname, p1_yrow);
    part2(fname, p2_bbox);
}

fn part1(fname: &str, yrow: i32) {
    /* TODO - make timing function? */
    let start = Instant::now();
    let sbvec= parse_file(fname);
    debug!("parse_file took {:?} seconds", start.elapsed());

    let mut poly_vec: Vec<Polygon> = vec![];
    let mut poly_agg: MultiPolygon<f64> = MultiPolygon::new(vec![]);
    for (sbix, sb) in sbvec.iter().enumerate() {
        let sb_poly = sb.multi_poly();
        poly_vec.push(sb.poly());
        // println!("sbix = {sbix}");
        // poly_print(&sb_poly.0[0], "sb_poly");
        let tmp_poly = poly_union(&poly_agg, &sb_poly);
        poly_agg = tmp_poly;
        // let tmps = format!("poly_agg_partial_{sbix:?}");
        // multi_poly_draw(&poly_agg, tmps.as_str(), bbarg);
    }
    multi_poly_draw(&poly_agg, "poly_agg", (0, 4000000));

    let (minx, maxx) = multi_poly_xlims(&poly_agg);
    let eps = 0.1;
    let row_poly = polygon![
        (x: minx, y: yrow as f64-0.5_f64),
        (x: maxx, y: yrow as f64-0.5_f64),
        (x: maxx, y: yrow as f64+0.5_f64),
        (x: minx, y: yrow as f64+0.5_f64)
    ];
    debug!("yrow = {yrow}");
    let row_mpoly= MultiPolygon::from_iter(vec![row_poly]);
    // multi_poly_draw(&row_mpoly, "row_mpoly", (0, 4000000));
    let inter = poly_agg.intersection(&row_mpoly);
    debug!("inter = {inter:?}");
    // this seems to work w/o running into edge cases, which is somewhat surprising 
    // this could just be an artifact of this particular data
    println!("part1: row_poly inter area = {:?}", inter.unsigned_area().ceil());
}


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

fn multi_poly_xlims(mp: &MultiPolygon) -> (f64, f64) {
    let epts: Vec<Coord> = mp.exterior_coords_iter().collect();
    let minx = epts.iter()
        .map(|ept| ept.x)
        .reduce(|e1, e2| e1.min(e2))
        .unwrap();
    let maxx = epts.iter()
        .map(|ept| ept.x)
        .reduce(|e1, e2| e1.max(e2))
        .unwrap();
    return (minx, maxx);
}

fn multi_poly_ylims(mp: &MultiPolygon) -> (f64, f64) {
    let epts: Vec<Coord> = mp.exterior_coords_iter().collect();
    let miny = epts.iter()
        .map(|ept| ept.y)
        .reduce(|e1, e2| e1.min(e2))
        .unwrap();
    let maxy = epts.iter()
        .map(|ept| ept.y)
        .reduce(|e1, e2| e1.max(e2))
        .unwrap();
    return (miny, maxy);
}

fn multi_poly_draw(mp: &MultiPolygon, name: &str, bbarg: (XYNum, XYNum)) {
    let fname = format!("day15_{name}.svg");
    let drawing_area = 
        SVGBackend::new(fname.as_str(), (500, 500))
        .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let (minx, maxx) = multi_poly_xlims(mp);
    let (miny, maxy) = multi_poly_ylims(mp);
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
    // let tmp = p1.union(&p2);
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
        let pts = self.points().map(|xy| { (xy.x as f64, xy.y as f64)});
        let line_string: LineString = LineString::from_iter( pts.into_iter() );
        let poly = Polygon::new(line_string, vec![]);
        let out = MultiPolygon::from(vec![poly]);
        return out;
    }
    pub fn poly(&self) -> Polygon {
        let pts = self.points().map(|xy| { (xy.x as f64, xy.y as f64)});
        let line_string: LineString = LineString::from_iter( pts.into_iter() );
        return Polygon::new(line_string, vec![]);
    }
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

