
// use core::num::dec2flt::float;
// use std::ops;
use std::process;
use std::collections::{HashMap, VecDeque, BTreeMap};
use regex::{Regex, internal::Inst};
use std::time::{Duration, Instant};
use crate::utils::read_lines;

// TODO - look into logger
const DEBUG: bool = false;
const P1_ROUNDS: usize = 20;
const P2_ROUNDS: usize = 10000;

pub fn run(fname: &str) {
    let monkeys: Vec<Monkey> = parse_input(fname);

    let mut m1: Vec<Monkey> = monkeys.clone();
    partx(&mut m1, true);

    let mut m2: Vec<Monkey> = monkeys;
    partx(&mut m2, false);
}

fn partx(mvec: &mut Vec<Monkey>, part1: bool) {
    let part2: bool = !part1;
    for mix in 0..mvec.len() {
        println!("{:?}", mvec[mix]);
    }

    let num_rounds = if part1 {P1_ROUNDS} else {P2_ROUNDS};
    let start = Instant::now();
    
    for round in 0..num_rounds {
        // TODO - use a logger w/ debug???
        if DEBUG { println!("<<<<<<<<<<<<<<<< ROUND >>>>>>>>>>>>>>>> {}", round+1); }
        for mix in 0..mvec.len() {
            if DEBUG {println!("\n  MONKEY{mix}:");}
            // let m: &Monkey = &mvec[mix];
            // let items = &(mvec[mix].items);
            while mvec[mix].items.len() > 0 {
                if let Some(naive) = mvec[mix].items[0].naive {
                    println!("\n    V: {:?}", naive);
                }
                let (op, op_arg) = (mvec[mix].op, mvec[mix].op_arg);
                let test_div = mvec[mix].test_div;

                mvec[mix].items[0].op(op, op_arg);

                // mvec[mix].items[0].div(3);
                if part1 {
                    mvec[mix].items[0].op(Op::Div, 3);
                }
                let test_out = mvec[mix].items[0].div_by(test_div);
                let dst = if test_out {mvec[mix].dst_true} else {mvec[mix].dst_false};

                // remove item from current monkey, and put new result to dst
                // println!("{mix} -> {dst}: itm={itm}, tmp1={tmp1}, tmp2={tmp2}, test_out={test_out}");
                let itm = mvec[mix].items.pop_front().unwrap();
                mvec[mix].inspect += 1;
                mvec[dst].items.push_back(itm);

                // if mix == 2 {
                    // println!("  Monkey inspects an item with a worry level of {itm}.");
                    // println!("    Worry level is {} by {} to {}.", mvec[mix].op, mvec[mix].op_arg, tmp1);
                    // println!("    Monkey gets bored with item. Worry level is divided by 3 to {tmp2}.");
                    // println!("    Current worry level is {} by {} = {}.", mvec[mix].test, mvec[mix].test_div, test_out);
                    // println!("    Item with worry level {tmp2} is thrown to monkey {dst}.");
                // }
            } /* end: while mvec[mix].items.len() > 0 */
        } /* end: for mix in 0..mvec.len() */

        let round_checks: [usize; 16] = [1, 20, 50, 100, 200, 500, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000];
        if round_checks.contains(&(round+1)) {
            println!("== After round {} == ({:?} s)", round+1, start.elapsed());
            for mix in 0..mvec.len() {
                println!("Monkey {mix} inspected items {} times.", mvec[mix].inspect);
            }
        }
    } /* end: for round in 0..num_rounds */

    let mut inspect_vec: Vec<i64> = vec![];
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

/*
fn op_fn(op: &str, x: &i64, op_arg: &String) -> Option<i64> {
    let mut op: &str = op;
    let (op, op_arg): (&str, Option<i64>) = {
        if let Ok(ok_int) = op_arg.parse::<i64>() {
            (op, Some(i64::from(ok_int)))
        }
        else if op_arg.as_str() == "old" && op == "*" {
            ("^", Some(i64::from(2)))
        }
        else {
            ("", None)
        }
    };

    if let Some(ok_arg) = op_arg {
        // println!("x = {x}, ok_arg = {ok_arg}");
        return Some(
            match op {
                "+" => x + ok_arg,
                "-" => x - ok_arg,
                "*" => x * ok_arg,
                "/" => x / ok_arg,
                "^" => x.clone() * x, //.pow(ok_arg as u32),
                 _  => x.clone()
            }
        );
    }
    
    return None;
}
*/

#[derive(Debug, Copy, Clone)]
enum Op{ Add, Mul, Div, Pow }

#[derive(Debug, Clone)]
struct Item {
    naive: Option<i64>,
    ival: i64,
    imod: i64,
    factors: BTreeMap<i64, bool>
}

impl Item {
    pub fn from(v: i64) -> Item {
        let mut factors: BTreeMap<i64, bool> = BTreeMap::new();
        let mut imod: i64 = 1;
        for x in [2i64, 3, 5, 7, 11, 13, 17, 19, 23] {
            imod *= x;
            factors.insert( x, 0 == v % x );
        }
        let naive = if DEBUG { Some(v) } else {None};
        return Item{ naive: naive, ival: v % imod, imod: imod, factors: factors };
    }

    fn update_per_ival(&mut self) {
        for k in self.factors.clone().keys() {
            *(self.factors.get_mut(k)).unwrap() = self.ival % k == 0;
        }
    }
 

    // update factors w/ multiply
    pub fn mul(&mut self, m: i64) {
        if let Some(naive) = self.naive { self.naive = Some(naive * m) };
        self.ival *= m;
        self.ival %= self.imod;
        self.update_per_ival();
    }
    // update factors w/ add 
    pub fn add(&mut self, a: i64) {
        if let Some(naive) = self.naive { self.naive = Some(naive + a) };
        self.ival += a;
        self.ival %= self.imod;
        self.update_per_ival();
    }
    // update factors w/ pow
    pub fn pow(&mut self, _: i64) { 
        /* simple hack to enable square for now, FIXME: assumes all pow are square */
        if let Some(naive) = self.naive { self.naive = Some(naive * naive) };
        self.ival *= self.ival;
        self.ival %= self.imod;
        self.update_per_ival();
    }
    // util divide
    pub fn div(&mut self, d: i64) {
        if let Some(naive) = self.naive { self.naive = Some(naive / d) };
        self.ival /= d; // *self.ival;
        self.ival %= self.imod;
        self.update_per_ival();
    }

    fn naive_factors_print(&self) {
        if let Some(naive) = self.naive {
            println!("      DBG: naive = {naive}");
            for k in self.factors.keys() {
                if (naive % k == 0) != self.factors[k] {
                    println!("        DISAGREE!!!: k = {k}, ival={}, imod={}, {naive} % {k} = {}, self.factors[{k}] = {}",
                        self.ival, self.imod,
                        naive % k, self.factors[k]
                    );
                    process::exit(1);
                }
            }
        }
    }

    pub fn op(&mut self, op: Op, op_arg: i64) {
        if let Some(_naive) = self.naive {
            println!("      DBG: op = {op:?}, op_arg = {op_arg}");
        }
        self.naive_factors_print();
        match op {
            Op::Mul => { self.mul(op_arg) },
            Op::Add => { self.add(op_arg) },
            Op::Div => { self.div(op_arg) },
            Op::Pow => { self.pow(op_arg) }
        }
        self.naive_factors_print();
    }

    pub fn div_by(&self, d: i64) -> bool {
        let mut out: bool = false;
        if let Some(ok_bool) = self.factors.get(&d) {
            out = *ok_bool;
        }
        if DEBUG {
            if let Some(naive) = self.naive {
                if (naive % d == 0) != out {
                    for _ix in 0..10 {
                        println!("DANGER!");
                    }
                }
                println!("      DBG: div_by: naive={naive}, {naive} % {d} = {}, out = {out}", naive % d);
            }
        }
        return out;
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    mky_ix: usize,
    inspect: i64,
    items: VecDeque<Item>,
    op: Op, /* string is simplest, use w/ op_fn */
    op_arg: i64,
    test_div: i64,
    dst_true: usize,
    dst_false: usize 
}

fn parse_input(fname: &str) -> Vec<Monkey> {
    let mut out: Vec<Monkey> = vec![];
    let mut mky_pat: String = String::from(r"(?s)"); 
    mky_pat += r".*Monkey (?P<mky_ix>\d+):";
    mky_pat += r".*Starting items: (?P<items>.+)";
    mky_pat += r".*Operation: new = old (?P<op>\S+) (?P<op_arg>\S+)";
    mky_pat += r".*Test: (?P<test>\S+) by (?P<test_div>\d+)";
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
                    // println!("line_agg =\n-->{line_agg}<--");
                    if let Some(caps) = mky_re.captures(&line_agg) {
                        // println!("caps = \n{caps:?}");
                        // println!("Make monkey here!\n");
                        let items_vec: VecDeque<Item> = 
                            caps.name("items").unwrap().as_str().trim()
                            .split(", ").map(
                                |s|
                                Item::from( s.parse().expect("parse error") )
                            )
                            .collect();

                        // let test_key = String::from(caps.name("test").unwrap().as_str());
                        let op_lu: HashMap<&str, Op> = HashMap::from([
                            ("+", Op::Add),
                            ("*", Op::Mul),
                            ("^", Op::Pow),
                        ]);
                        let (op_str, op_arg): (&str, i64) = {
                            let tmp_op: &str = caps.name("op").unwrap().as_str();
                            let tmp_op_arg: &str= caps.name("op_arg").unwrap().as_str();
                            if tmp_op_arg == "old" {
                                ("^", 1)
                            }
                            else {
                                ( 
                                    tmp_op, 
                                    caps.name("op_arg").unwrap().as_str().parse().unwrap()
                                )
                            }
                        };

 
                        let mky = Monkey {
                            mky_ix:    caps.name("mky_ix").unwrap().as_str().parse().unwrap(),
                            inspect:   0,
                            items:     items_vec,
                            op:        op_lu.get(op_str).unwrap().clone(),
                            op_arg:    op_arg,
                            test_div:  caps.name("test_div").unwrap().as_str().parse().unwrap(),
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
