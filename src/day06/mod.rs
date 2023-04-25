
use crate::utils::read_lines;
use std::collections::HashSet;
// use std::io::{BufReader, Read};
// use std::io::prelude::*;

pub fn run(fname: &str) {
    let (sop, som) = part12(fname);
    println!("sop = {:?}, som = {:?}", sop, som);
}

const SOPN: usize =  4;
const SOMN: usize = 14;
fn part12(fname: &str) -> (Option<usize>, Option<usize>) {
    let mut out = (None, None);
    let mut sop: [char; SOPN] = ['1', '2', '3', '4'];
    let mut som: [char; SOMN] = ['0'; SOMN];
    let mut ix = 0;
    if let Ok(lines) = read_lines(fname){
        for line in lines {
            if let Ok(ok_line) = line {
                for c in ok_line.chars() {
                    if out.0 == None {
                        sop[ix % SOPN] = c;
                        if (ix >= SOPN) & (SOPN == HashSet::from(sop).len()) {
                            out.0 = Some(ix+1);
                        }
                    }
                    if out.1 == None {
                        som[ix % SOMN] = c;
                        if (ix >= SOMN) & (SOMN == HashSet::from(som).len()) {
                            out.1 = Some(ix+1);
                        }
                    }
                    ix += 1;
                }
            }
        }
    }
    return out;
}

/*
fn buf_reader(fname: &str) -> Result<BufReader<File>, std::io::Error> {
    let mut bufrd = BufReader::new(file);
    return Ok(bufrd);
} */
