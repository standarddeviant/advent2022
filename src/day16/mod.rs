use std::str::FromStr;
use std::hash::{Hash};
use std::collections::HashMap;
// use egui::epaint::ahash::HashMap;
use log::{debug, info};
use regex::Regex;
use crate::utils::read_lines;
use pathfinding::prelude::{dijkstra};//, dijkstra_all};
// use egui::plot::{Line, Plot, PlotPoints};

pub fn run(fname: &str) {
    info!("fname = {fname}");
    part1(fname, "AA");

    /*
    let sin: PlotPoints = (0..1000).map(|i| {
        let x = i as f64 * 0.01;
        [x, x.sin()]
    }).collect();
    let line = Line::new(sin);
    Plot::new("my_plot").view_aspect(2.0).show(plot_ui, |plot_ui| plot_ui.line(line));
    */
}

fn part1(fname: &str, start_key: &str) {
    let valves = parse_file(fname);
    let vmap_nz = make_nonzero_graph(&valves, &start_key.into());
    println!("part1(WIP): vmap_nz = \n");
    for (k, v) in &vmap_nz {
        println!("k = {k}");
        println!("v = {v:?}\n");
    }
    // println!("vmap_nz = {vmap_nz:?}");
}

fn make_nonzero_graph(valves: &Vec<Valve>, start_key: &String) -> HashMap<String, Valve> {
    info!("valves.len() = {}", valves.len());
    for (ix, v) in valves.iter().enumerate() {
        info!("valve [{ix:2}] = {v:?}");
    }

    let vmap_all: HashMap<String, Valve> = HashMap::from_iter(
        valves.iter()
        .map(|v| (v.name.clone(), v.clone()))
    );

    fn leads_func(a: &Valve, vm: &HashMap<String, Valve>) -> Vec<(Valve, usize)> {
        a.leads.iter().map(
            |k|
            (vm[&k.0].clone(), k.1)
        ).collect()
    }

    // find cost of the shortest paths of initial graph to make a non-zero graph
    let mut vmap_nz: HashMap<String, Valve> = HashMap::new();
    for (ka, va) in &vmap_all {
        // ignore va if rate==0 and isn't the start
        if va.rate == 0 && !va.name.eq(start_key) { continue; }
        let mut val_nz: Valve = Valve { name: ka.clone(), rate: va.rate, leads: vec![] };
        for (kb, vb) in &vmap_all {
            // ignore vb if rate==0
            if vb.rate == 0 { continue; }
            let start = vmap_all[ka].clone();
            let end = vmap_all[kb].clone();
            let shortest_path = dijkstra(
                &start, 
                |v| leads_func(v, &vmap_all),
                |v| *v == end
            ).unwrap();
            // add path val_nz w/ associated cost via dijkstra
            val_nz.leads.push((kb.clone(), shortest_path.1));
        }
        vmap_nz.insert(ka.clone(), val_nz);
    }
    return vmap_nz;
}

fn parse_file(fname: &str) -> Vec<Valve> {
    let mut out: Vec<Valve> = vec![];
    if let Ok(lines) = read_lines(fname) {
        // println!("lines = {lines:?}");
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(okline) = line {
                let v: Result<Valve, ValveParseError> = okline.parse();
                if let Ok(okv) = v {
                    out.push(okv);
                }
            }
        }
    }
    // out.sort_by(|a, b| b.rate.cmp(&a.rate));
    return out;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    rate: i32,
    leads: Vec<(String, usize)>
}


/*
impl Hash for Valve {
    fn hash<H>(&self, _: &mut H) where H: Hasher {
        // todo!()
    }
} */


#[derive(Debug, PartialEq, Eq)]
struct ValveParseError;
impl FromStr for Valve {
    type Err = ValveParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        debug!("Valve::from_str: s = {s}");
        let repat = Regex::new(
            r"Valve (?P<name>\S+) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<leads_str>.+)"
        ).unwrap();
        if let Some(caps) = repat.captures(&s) {
            debug!("caps = {caps:?}");
            let name: String = String::from(caps.name("name").unwrap().as_str());
            let rate: i32 = caps.name("rate").unwrap().as_str().parse().unwrap();
            let leads: Vec<(String, usize)> = caps.name("leads_str").unwrap().as_str()
                .split(", ")
                .map(|x| (String::from(x), 1))
                .collect();
            let valve = Valve{name: name, rate: rate, leads: leads};
            return Ok(valve);
        }
        return Err(ValveParseError);
    }
}
