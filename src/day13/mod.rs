use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fs;
// use std::str::FromStr;
// use crate::utils::read_lines;
use serde_json::{Value, json};
use serde_json::Value::{Number, Array};

pub fn run(fname: &str) {
    part1(fname);
    part2(fname);
}

fn part1(fname: &str) {
    // parse pairs
    let pairs = parse_input(fname);

    // init accum variable
    let mut true_ix_sum = 0;
    for (ix, pair) in pairs.iter().enumerate() {
        if pair.len() != 2 { continue; }
        let (a, b) = (&pair[0], &pair[1]);
        if let Some(ltval) = lt(a, b) {
            if ltval {
                true_ix_sum += ix+1;
            }
        }
    }
    println!("part1: true_ix_sum = {true_ix_sum}");
}

fn part2(fname: &str) {
    // parse pairs
    let mut pairs: Vec<Vec<Value>> = parse_input(fname);

    // add divider packets
    let div_pkts = vec![ json!([[2]]), json!([[6]])];
    pairs.push(div_pkts.clone());

    // put pairs in singles 
    let mut singles: Vec<Value> = vec![];
    for pair in pairs {
        singles.extend(pair);
    }

    // sort singles by lt function and Ordering
    singles.sort_by(|a, b| -> Ordering {
        match lt(a, b) {
            Some(true ) => Ordering::Less,
            Some(false) => Ordering::Greater,
            _           => Ordering::Equal
        }
    });

    // find divider packets, record 1-based index
    let mut div_pkts_ixs: Vec<usize> = vec![];
    for (ix, s) in singles.iter().enumerate() {
        if div_pkts.contains(s) {
            println!("sorted single {ix:2} = {s:?}");
            div_pkts_ixs.push(ix+1);
        }
    }

    // multiply divider packet indices (1 based)
    println!("part2: dividier packet indices (1 based) product = {}", 
        div_pkts_ixs[0] * div_pkts_ixs[1]);

}


/* TODO - make pair into proper type w/ a and b? */
/* TODO - return Result instead of Option here.... */
/* TODO - make this a proper override of the less-than symbol, '<' */
fn lt(basea: &Value, baseb: &Value) -> Option<bool> {
    match (basea, baseb) {
        (Value::Number(na), Value::Number(nb)) => {
            let (na, nb) = (na.as_f64()?, nb.as_f64()?);
            // println!("      DBG num/num lt!:\n    a={na:?}\n    b={nb:?}");
            if na < nb { return Some(true)  }
            if na > nb { return Some(false) }
            return None
        },
        (Array(va), Array(vb)) => {
            // convert to dequeues to conveniently pop_front
            let mut vda: VecDeque<&Value> = VecDeque::from_iter(va);
            let mut vdb: VecDeque<&Value> = VecDeque::from_iter(vb);
            loop {
                /* NOTE: these checks could be at the top or bottom of loop,
                         but an array may arrive into this function empty
                         from the start */
                /* return true (i.e. a < b) if a is empty and b still contains items */
                if !vda.is_empty() &&  vdb.is_empty() { return Some(false); }
                /* return false (i.e. a > b) if b is empty and a still contains items */
                if  vda.is_empty() && !vdb.is_empty() { return Some(true); }
                /* take an item from each list */

                let (tmpa, tmpb) = ( vda.pop_front()?, vdb.pop_front()? );
                if let Some(tmpbool) = lt(tmpa, tmpb) {
                    return Some(tmpbool);
                }
            }
        },
        // if mismatched types, promote num to vec of len 1, then recall to array/array
        (Value::Array(_va), Value::Number(nb)) => {
            let vb = Array(vec![Number(nb.clone())]);
            return lt(basea, &vb);
        },
        (Value::Number(na), Value::Array(_vb)) => {
            let va = Array(vec![Number(na.clone())]);
            return lt(&va, baseb);
        },
        _ => {
            None // unknown types?
        }
    }
}

fn parse_pair(pstr: &str) -> Option<Vec<Value>> {
    let ab: Vec<&str> = pstr.split("\n").collect();
    let ab: Vec<Value> = ab.iter()
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();
    // println!("ab = {ab:?}");
    Some(ab)
}

fn parse_input(fname: &str) -> Vec<Vec<Value>> {
    let mut out: Vec<Vec<Value>> = vec![];
    let fstr = fs::read_to_string(fname)
        .expect(format!("File '{fname}' not readable.").as_str());
    // let mut pairs: Vec<Pair> = vec![];

    let pair_strings: Vec<&str> = fstr.split("\n\n")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    for pstr in pair_strings {
        // let tmp: Pair = pstr.parse().unwrap();
        // println!("What to do w/ pstr (JSON?):\n{pstr}");
        if let Some(pair) = parse_pair(pstr) {
            out.push(pair);
        };
    }
    return out;
}