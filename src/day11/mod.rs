
// use core::num::dec2flt::float;
use std::collections::{HashMap, VecDeque};
use regex::Regex;
use crate::utils::read_lines;

pub fn run(fname: &str) {
    let mut monkeys = parse_input(fname);

    part1(&mut monkeys);
}

fn part1(mvec: &mut Vec<Monkey>) {
    for mix in 0..mvec.len() {
        println!("{:?}", mvec[mix]);
    }
    
    for round in 0..20 {
        for mix in 0..mvec.len() {
            // let m: &Monkey = &mvec[mix];
            // let items = &(mvec[mix].items);
            while mvec[mix].items.len() > 0 {
                let itm: i32 = mvec[mix].items[0];
                let tmp1: i32 = op_fn(&mvec[mix].op, itm, &mvec[mix].op_arg).unwrap();
                let tmp2: i32 = ((tmp1 as f32) / 3.0_f32).floor() as i32;
                let test_out = test_fn(&mvec[mix].test, tmp2, mvec[mix].test_arg);
                let dst = if test_out {mvec[mix].dst_true} else {mvec[mix].dst_false};
                // remove item from current monkey, and put new result to dst
                println!("{mix} -> {dst}: itm={itm}, tmp1={tmp1}, tmp2={tmp2}, test_out={test_out}");
                mvec[mix].items.pop_front();
                mvec[mix].inspect += 1;
                mvec[dst].items.push_back(tmp2);
            }
        }
    }

    let mut inspect_vec: Vec<i32> = vec![];
    for mix in 0..mvec.len() {
        let (mky_ix, inspect) = (mvec[mix].mky_ix, mvec[mix].inspect);
        inspect_vec.push(inspect);
        println!("for {mky_ix:2} -> inspect = {inspect:3}");
    }
    inspect_vec.sort();
    inspect_vec.reverse();
    println!("inspect_vec = {inspect_vec:?}");

    let monkey_business = inspect_vec[0] * inspect_vec[1];
    println!("part1: monkey_business = {monkey_business}")
}

fn op_fn(op: &str, x: i32, op_arg: &String) -> Option<i32> {
    let mut op: &str = op;
    let (op, op_arg): (&str, Option<i32>) = {
        if let Ok(ok_int) = op_arg.parse::<i32>() {
            (op, Some(ok_int))
        }
        else if op_arg.as_str() == "old" && op == "*" {
            ("^", Some(2))
        }
        else {
            ("", None)
        }
    };

    if let Some(ok_arg) = op_arg {
        return Some(
            match op {
                "+" => x + ok_arg,
                "-" => x - ok_arg,
                "*" => x * ok_arg,
                "/" => x / ok_arg,
                "^" => x ^ ok_arg,
                 _  => x
            }
        );
    }
    
    return None;
}

fn test_fn(test: &str, x: i32, y: i32) -> bool {
    match test {
        "divisible" => x % y == 0,
                 _  => false
    }
}

#[derive(Debug)]
struct Monkey {
    mky_ix: i32,
    inspect: i32,
    items: VecDeque<i32>,
    op: String, /* string is simplest, use w/ op_fn */
    op_arg: String,
    test: String, /* string is simplest, use w/ test_fn */
    test_arg: i32,
    dst_true: usize,
    dst_false: usize 
}

fn parse_input(fname: &str) -> Vec<Monkey> {
    let mut out: Vec<Monkey> = vec![];
    let mut mky_pat: String = String::from(r"(?s)"); 
    mky_pat += r".*Monkey (?P<mky_ix>\d+):";
    mky_pat += r".*Starting items: (?P<items>.+)";
    mky_pat += r".*Operation: new = old (?P<op>\S+) (?P<op_arg>\S+)";
    mky_pat += r".*Test: (?P<test>\S+) by (?P<test_arg>\d+)";
    mky_pat += r".*If true: throw to monkey (?P<dst_true>\d+)";
    mky_pat += r".*If false: throw to monkey (?P<dst_false>\d+).*";
    let mky_re = Regex::new(&mky_pat).unwrap();

    let mut line_agg = String::new();
    if let Ok(lines) = read_lines(fname) {
        for (ix, tbd_line) in lines.enumerate() {
            let ixmod = ix % 7;
            if let Ok(ok_line) = tbd_line {
                if ixmod == 0 {
                    line_agg.clear();
                }
                if ixmod <= 5 {
                    line_agg += &ok_line;
                }

                /* TODO - reduce how nesty this is */
                // else if ixmod == 6 {
                if ok_line.contains("If false") {
                    println!("line_agg =\n-->{line_agg}<--");
                    if let Some(caps) = mky_re.captures(&line_agg) {
                        // println!("caps = \n{caps:?}");
                        // println!("Make monkey here!\n");
                        let items_vec: VecDeque<i32> = 
                            caps.name("items").unwrap().as_str().trim()
                            .split(", ").map(|s| s.parse()
                            .expect("parse error"))
                            .collect();

                        // let test_key = String::from(caps.name("test").unwrap().as_str());
                        let mky = Monkey {
                            mky_ix:    caps.name("mky_ix").unwrap().as_str().parse().unwrap(),
                            inspect:   0,
                            items:     items_vec,
                            op:        String::from(caps.name("op").unwrap().as_str()),
                            op_arg:    String::from(caps.name("op_arg").unwrap().as_str()),
                            test:      String::from(caps.name("test").unwrap().as_str()),
                            test_arg:  caps.name("test_arg").unwrap().as_str().parse().unwrap(),
                            dst_true:  caps.name("dst_true").unwrap().as_str().parse().unwrap(),
                            dst_false: caps.name("dst_false").unwrap().as_str().parse().unwrap(),
                        };
                        // println!("{:?}", &mky);
                        out.push(mky);
                    } /* end: if let Some(caps) = mky_re.captures(&line_agg) { */

                    /* clear line_agg after regex attempt */
                    line_agg.clear();
                } /* else if ixmod == 6 */
            } /* end: if let Ok(ok_line) = tbd_line */
        } /* end: for (ix, tbd_line) in lines.enumerate() */
    } /* end: if let Ok(lines) = read_lines(fname) */

    return out;
}
