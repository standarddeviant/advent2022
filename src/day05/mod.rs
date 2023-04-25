use std::{collections::HashMap, ops::Range};

// use std::collections::HashMap;
use regex::Regex;
// use std::collections::HashMap;
use crate::utils::read_lines;

pub fn run(fname: &str) {
    let mut part1: Vec<char> = vec![];
    let mut part2: Vec<char> = vec![];
    /* part 1 */
    let (mut stacks, moves) = parse_input(fname);
    for m in moves {
       for _ in 0..m.count {
            let c: Option<char> = stacks
                .get_mut(&m.from).expect("hmmmm").pop();
            if let Some(to) = stacks.get_mut(&m.to) {
                if let Some(some_c) = c {
                    to.push(some_c);
                }
            }
       }
    }
    for ix in 0..9 {
        println!("part1: {} -> {:?}", ix, stacks.get(&ix).unwrap());
        part1.push(stacks.get_mut(&ix).unwrap().pop().unwrap());
    }
    println!("");

    /* part 2 */
    let (mut stacks, moves) = parse_input(fname);
    for m in moves {
        let r: Range<usize> = Range{
            start: stacks.get(&m.from).expect("msg").len() - m.count as usize,
            end  : stacks.get(&m.from).expect("msg").len() as usize
        };
        let mut v: Vec<char> = 
            stacks
            .get_mut(&m.from).expect("hmmmm")
            .drain(r)
            .into_iter()
            .collect();
        if let Some(to) = stacks.get_mut(&m.to) {
            to.append(&mut v);
        }
    }
    for ix in 0..9 {
        println!("part2: {} -> {:?}", ix, stacks.get(&ix).unwrap());
        part2.push(stacks.get_mut(&ix).unwrap().pop().unwrap());
    }

    println!("\npart1: {:?}", part1);
    println!("part2: {:?}", part2);
}

#[derive(Debug)]
struct Move {
    count: i32,
    from: i32,
    to: i32 
}

impl Move {
    pub fn from_string(in_str: &str) -> Result<Move, &str> {
        let re = Regex::new(
        r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)")
        .unwrap();
        let caps = re.captures(&in_str);
        return match caps {
            Some(some_caps) => {
                // println!("some_caps = {:?}", some_caps);
                Ok(Move{
                    count: some_caps["count"].parse::<i32>().unwrap(),
                    from : some_caps["from" ].parse::<i32>().unwrap() - 1,
                    to   : some_caps["to"   ].parse::<i32>().unwrap() - 1,
                })
            },
            None => Err("Regex didn't match")
        }
    }
}

fn parse_input(fname: &str) -> (HashMap<i32, Vec<char>>, Vec<Move>) {
    let N = 9;
    let mut out_stacks: HashMap<i32, Vec<char>> = (0..N).map(|x| (x, vec![])).collect();
    let mut out_moves: Vec<Move> = vec![];
    let mut first8: Vec<String> = vec![];
    if let Ok(mut lines_iter) = read_lines(fname) {
        for (ix, line) in lines_iter.enumerate() {
            if let Ok(line_str) = line {
                if ix < 9 {
                    first8.push(line_str);
                }
                else {
                    if let Ok(m) = Move::from_string(&line_str) {
                        out_moves.push(m);
                    }
               }
            } /* end if let Ok(mut lines_iter) = read_lines(...) */
        }
    }

    // ignore first pop; (numeric labels)
    first8.pop();
    for ix in 0..N {
        // &out_stacks[ix] = Vec<char>::from([]);
        out_stacks.insert(ix, vec![]);
    }

    // set number of stacks and init stacks
    for _ in 0..first8.len() {
        if let Some(line) = first8.pop() {
            for ix in 0..N {
                let uix: usize = ix as usize;
                if let Some(c) = line.chars().nth(1+uix*4) {
                    if c != ' ' {
                        if let Some(vto) = out_stacks.get_mut(&ix) {
                            vto.push(c);
                        }
                        // println!("{} -> {}", ix, c);
                    }
                }
           }
        }
    }
    return (out_stacks, out_moves);
}
