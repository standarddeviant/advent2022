use std::collections::VecDeque;
use std::fs;
use std::str::FromStr;
use regex::Regex;
// use crate::utils::read_lines;
use serde_json::Value;
use serde_json::Value::{Number, Array};


pub fn run(fname: &str) {
    let pairs = parse_input(fname);
    for pair in &pairs {
        if pair.len() != 2 { continue; }
        let (a, b) = (&pair[0], &pair[1]);
        if let Some(ltval) = lt(a, b) {
            println!("lt = {ltval}!!!!");
        }
    }
}


/* TODO - make pair into proper type w/ a and b? */
/* TODO - return Result instead of Option here.... */
fn lt(basea: &Value, baseb: &Value) -> Option<bool> {
    let mut ix0 = 0;
    /* working vars */
    let (mut ma, mut mb) = (basea, baseb);
    match (ma, mb) {
        (Value::Number(na), Value::Number(nb)) => {
            let (na, nb) = (na.as_f64()?, nb.as_f64()?);
            if na < nb { return Some(true)  }
            if na > nb { return Some(false) }
            return None
        },
        (Array(va), Array(vb)) => {
            let mut vda: VecDeque<&Value> = VecDeque::from_iter(va);
            let mut vdb: VecDeque<&Value> = VecDeque::from_iter(vb);
            let (mut ixa, mut ixb) = (0, 0);
            println!("    TODO array/array lt!:\n    a={vda:?}\n    b={vdb:?}");
            for itma in vda {
                println!("    itma = {itma:?}");
            }
            return None // FIXME
        },
        // if mismatched types, promote num to vec of len 1, then recall to array/array
        (Value::Array(va), Value::Number(nb)) => {
            let vb = Array(vec![Number(nb.clone())]);
            println!("    DBG: promoted {nb} to {vb}");
            return lt(ma, &vb);
        },
        (Value::Number(na), Value::Array(vb)) => {
            let va = Array(vec![Number(na.clone())]);
            println!("    DBG: promoted {na:?} to {va}");
            return lt(&va, mb);
        },
        _ => {
            println!("Unhandled lt!:\n    a={ma:?}\n    b={mb:?}");
            None // FIXME
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

/*
#[derive(Debug, Clone, Copy)]
enum Value { Number(i32), Depth(i32) }

fn ltvv(a: Vec<Value>, b: Vec<Value>) -> bool {
    struct LTState{n: i32, ig: usize, id: usize, depth: usize};
    let mut sa: LTState = LTState { n: -1, ig: 0, id: 0, depth: 0 };
    let mut sb: LTState = LTState { n: -1, ig: 0, id: 0, depth: 0 };
    while sa.ig < x.len() && sb.ig < y.len() {
        let (va, vb) = (a[sa.ig], b[sb.ig]);
        match va { 
            Value::Depth(dx) => {
                sa.depth += dx,
            

        }
        if sa.id==0 && sb.id == 0 {

        }
        while sa.depth != 
        if sa.id == 0 && sb.id == 0 {
            match 


        }
            if sa.depth < sb.depth
        }
            match (va, vb) {
                (Value::Number(na), Value::Number(nb)) => {
                    if na < nb { return true;  }
                    if na > nb { return false; }
                },
                (Value::Depth(dx), Value::Depth(dy)) => {
                    sa.depth += dx;
                    sb.depth += dy;
                }
                // (Value)
                (_, _) => {}
            }
        }
        if sa.depth != sb.depth {
            /* if left side ran out first, we're good, right? */
            if sa.depth < sb.depth {
                return true;
            }
            else if sa.depth > sb.depth {

            }
        }
            /* resolve depths to match */
   return false;
}

struct Pair {
    a: Vec<Value>,
    b: Vec<Value>
}

struct ItemParseError;

fn parse_item(s: &str) -> Result<Vec<Value>, ItemParseError> {
    let re_p1 = Regex::new(r"\[").unwrap();
    let re_n1 = Regex::new(r"\]").unwrap();
    let re_int = Regex::new(r"\d+").unwrap();
    let p1Matches: Vec<(usize, Value)> = re_p1.find_iter(s)
        .map(|m| (m.start(), Value::Depth(1)) )
        .collect();
    let n1Matches: Vec<(usize, Value)> = re_n1.find_iter(s)
        .map(|m| (m.start(), Value::Depth(-1)) )
        .collect();
    let intMatches: Vec<(usize, Value)>  = re_int.find_iter(s)
        .map(
            |m|
            (m.start(), Value::Number(m.as_str().parse::<i32>().unwrap()))
        )
        .collect();

    // append and sort match vectors w/ index@m.0, value@m.1
    let mut matches: Vec<(usize, Value)> = p1Matches;
    matches.extend(n1Matches);
    matches.extend(intMatches);
    matches.sort_by(|a: &(usize, Value), b: &(usize, Value)| a.0.partial_cmp(&b.0).unwrap());
    // emit sorted values, given value@m.1
    let out = matches.iter().map(|m| m.1).collect();
    return Ok(out);
        
    /* 
    for (ix, p1) in p1Matches.iter().enumerate() {
        println!("p1[{ix}] = {p1:?}");
    }
    for (ix, n1) in n1Matches.iter().enumerate() {
        println!("n1[{ix}] = {n1:?}");
    }
    for (ix, int) in intMatches.iter().enumerate() {
        println!("int[{ix}] = {int:?}");
    }
    */
}


#[derive(Debug)]
struct ParsePairError;

impl FromStr for Pair {
    type Err = ParsePairError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ab: Vec<&str> = s.split("\n").collect();
        
        // println!("pair = -->\n{pair}\n<--");
        if ab.len() == 2 {
            // println!("yippee, ab = {:?}", ab);
            let a: Result<Vec<Value>, ItemParseError> = parse_item(ab[0]);
            let b: Result<Vec<Value>, ItemParseError> = parse_item(ab[1]);
            if let (Ok(a), Ok(b)) = (a, b) {
                // println!("    a = {a:?}");
                // println!("    b = {b:?}");
                return Ok(Pair{a: a, b: b});
            }
        } /* end: if ab.len() == 2 */
        // return Err(());
        return Err(ParsePairError);
    } /* end: fn from_str(s: &str) -> Result<Self, Self::Err> */
} /* end: impl FromStr for Pair */

/*
fn less_than(ain: &Value, bin: &Value) -> bool {
    let mut a = ain;
    let mut b = bin;
    /*
    */
    let mut ix = 0;
    loop 
    match (*a, *b) {
        (Value::Number(numa), Value::Number(numb)) => {
            if numa.as_i64() < numb.as_i64() {
                return true;
            }
            else if numa.as_i64() > numb.as_i64() {
                return false;
            }
            else {

            }
        },
        (Value::Array(arra), Value::Array(arrb)) => {
            loop {
                if ix < arra.len() && 
            }
            for itm in loop {


            }
            arra.as_i64() < arrb.as_i64()
        },
        (Value::Array(aa), Value::Number(nb)) => {
            aa.iter()
            less_than(a, b)
        },
        _ => false
    }
}
*/
*/

fn parse_input(fname: &str) -> Vec<Vec<Value>> {
    let mut out: Vec<Vec<Value>> = vec![];
    let fstr = fs::read_to_string(fname)
        .expect(format!("File '{fname}' not readable.").as_str());
    // let mut pairs: Vec<Pair> = vec![];

    let pair_strings: Vec<&str> = fstr.split("\n\n")
        .map(|L| L.trim())
        .filter(|L| !L.is_empty())
        .collect();

    for (ix, pstr) in pair_strings.iter().enumerate() {
        // let tmp: Pair = pstr.parse().unwrap();
        println!("What to do w/ pstr (JSON?):\n{pstr}");
        if let Some(pair) = parse_pair(pstr) {
            out.push(pair);
        };
    }
    return out;
}