#![allow(unused)]  // FIXME

use regex::{Regex};
use std::collections::HashMap;
use crate::utils::read_lines;

/*
#[derive(Debug, Clone, PartialEq)]
enum FD {
    FILE,
    DIR,
}
*/

pub fn run(fname: &str) {
    let parser = parse_input(fname);
    part1(&parser);
    part2(&parser);
}

fn part1(parser: &Day7Parser) {
    let mut ltcount: usize = 0;
    let mut ltsum: usize= 0;
    /*
    */
    for ix in 0..parser.items.len() {
        if parser.items[ix].isdir {
            if let Some(somesz) = parser.items[ix].size {
                if somesz <= 100000 {
                    // parser.print_entry(ix);
                    // print!("{ix}, ");
                    ltcount += 1;
                    ltsum += somesz;
                }
            }
            else {
                println!("Oh no, sizes not fully resolved!!!!!");
            }
        }
    }
    println!("part1: ltcount = {ltcount}, ltsum = {ltsum}");
}

fn part2(parser: &Day7Parser) {
    /* convenience constants */
    let disk_size: usize = 70000000;
    let disk_fill: usize = parser.items[0].size.unwrap();
    let disk_space:usize = disk_size - disk_fill;
    let needed_space: usize = 30000000;
    let needed_free = needed_space - disk_space;

    /* min search vars */
    let mut min_dir_size = disk_size;
    let mut min_dir_ix: usize = 0;
    for ix in 0..parser.items.len() {
        if let Some(somesz) = parser.items[ix].size {
            if (somesz < min_dir_size) && (somesz >= needed_free) {
                min_dir_ix = ix;
                min_dir_size = somesz;
            }
        }
    }

    println!("part2: ");
    parser.print_entry(min_dir_ix);
}

#[derive(Debug)]
struct FSItem {
    isdir: bool,
    children: Vec<usize>,
    size: Option<usize>
}
 
#[derive(Debug)]
struct Day7Parser {
    count: usize,
    pwdix: usize,
    pwdvec: Vec<String>,
    strmap: HashMap<String, usize>,
    intmap: HashMap<usize, String>,
    items: Vec<FSItem>
}

impl Day7Parser {
    pub fn new() -> Day7Parser {
        Day7Parser{
            count: 0,
            pwdix: 0,
            pwdvec: vec![String::from("")],
            strmap: HashMap::new(),
            intmap: HashMap::new(),
            items: vec![]
       }
    }

    pub fn pwdstr(&self) -> String {
        return self.pwdvec.join("/");
    }

    fn cd(&mut self, cd_arg: String) {
        // println!("INFO: handling cd_arg = {:?}", cd_arg);
        if cd_arg.eq("..") {
            self.pwdvec.pop(); /* TODO - check output */
        }
        else if cd_arg.eq("/") {
           self.pwdvec = vec![String::from("")]
        }
        else {
            self.pwdvec.push(String::from(cd_arg));
        }
        let pwdstr: String = self.pwdstr();
        if self.strmap.contains_key(&pwdstr) {
            /* we've already seen this, so update pwdix per strmap */
            if let Some(someix) = self.strmap.get(&pwdstr) {
                self.pwdix = *someix;
                // println!("    DBG: per cd operation, changing pwdix to {}", self.pwdix)
            }
            else {
                /* TODO - handle the bad news... */
            }
        }
        else {
            /* new dir, so add it... - could use register instead???? */
            // println!("    DBG: self.strmap.insert({}, {});", &pwdstr, self.count);
            self.strmap.insert(pwdstr.clone(), self.count);
            self.intmap.insert(self.count, pwdstr.clone());
            self.items.push(
                FSItem { 
                    isdir: true, children: vec![], size: None
                }
            );
            self.pwdix = self.count;
            self.count += 1;
        }
    }

    fn register(&mut self, isdir: bool, path: &String, sz: Option<usize>) {
        /* only register if we've never seen it before per strmap */
        if !self.strmap.contains_key(path) {
            /* add child if pwdix is valid as self.childrens index */
            if(self.pwdix < self.items.len()) {
                self.items[self.pwdix].children.push(self.count);
            }
            /*
            if let Some(chvec) = self.items[self.pwdix].children.push(self.count)
                chvec.push(self.count);
            } */
       
            /*
            println!("INFO: register \n    @ pwd = {:?}", path);
            println!("    w/ parent = ({}): {:?}", self.pwdix, parent);
            println!("    of size = {:?}", sz);
            println!("register: self.strmap.insert({}, {});", path, self.count);
            */
            self.strmap.insert(path.clone(), self.count);
            self.intmap.insert(self.count, path.clone());
            self.items.push(
                FSItem { 
                    isdir: isdir,
                    children: vec![],
                    size: sz 
                }
            );
            /* increment count */
            self.count += 1;
        }
    }

    fn parse_line(&mut self, line: &String) {
        let pat_cd = Regex::new(r"^\$\s+cd\s+(\S+)").unwrap();
        let pat_dir = Regex::new(r"^dir\s+(\S+)").unwrap();
        let pat_file= Regex::new(r"^(\d+)\s+(\S+)").unwrap();
        if let Some(caps) = pat_cd.captures(&line) {
            let cd_arg = caps.get(1).unwrap().as_str();
            self.cd(String::from(cd_arg));
        }
        else if let Some(caps) = pat_dir.captures(&line) {
            let dir_name = caps.get(1).unwrap().as_str();
            let parent_path = self.pwdstr();
            let dir_path = format!("{parent_path}/{dir_name}");
            self.register(true, &dir_path, None);
       }
        else if let Some(caps) = pat_file.captures(&line) {
            let sz_str= String::from(caps.get(1).unwrap().as_str());
            let file_name= String::from(caps.get(2).unwrap().as_str());
            let parent_path = self.pwdstr();
            let file_path = format!("{parent_path}/{file_name}");
            let sz: usize = sz_str.parse::<usize>().unwrap();
            self.register(false, &file_path, Some(sz));
       }
    }

    pub fn resolve_sizes(&mut self) {
        let mut pwdstk: Vec<usize> = vec![0];
        'outer: loop {
            /* if pwdixs is zero, then all paths sizes should be resolved */
            if pwdstk.len() == 0 {
                break;
            }
            // println!("pwdstk = {:?}", pwdstk);
            let pwd = pwdstk[pwdstk.len()-1];
            let pwd_children = &self.items[pwd].children;
            let mut pwdsz: usize = 0;
            /* loop over children */
            for luchix in 0..pwd_children.len() {
                let chix = pwd_children[luchix];
                if let Some(somesz) = self.items[chix].size {
                    pwdsz += somesz;
                }
                else {
                    /* getting here means a size is None, i.e. it's a dir size
                     * that hasn't been calculated yet */
                    pwdstk.push(chix);
                    continue 'outer;
                }
            }

            /* getting here means all child sizes have been accounted for,
             * so let's set this pwd's size to tmpsz and pop pwd from pwdstk*/
            self.items[pwd].size = Some(pwdsz);
            // println!("pwdsz = {pwdsz}");
            pwdstk.pop();
        } /* 'outer: loop */
    }

    pub fn print_entry(&self, ix: usize) {
        if ix < self.items.len() {
            let ch = &self.items[ix];
            println!("{ix:3}: d={}, sz={:?}, p={}, ch={:?}",
                ch.isdir,
                ch.size,
                self.intmap.get(&ix).unwrap(),
                ch.children
            );
        }
    }

}

fn parse_input(fname: &str) -> Day7Parser {
    // let mut out: Vec<_> = vec![];
    // let mut out: Vec<_> = vec![];

    let mut parser = Day7Parser::new();

    // let pwd_str: String = pvec.join("/");
    // let mut pwd_ix: usize = out.len() - 1;
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(ok_line) = line {
                parser.parse_line(&ok_line);
            }
        }
    }

    /*
    let tmpixs = [
        37, 39, 46, 60, 62, 65, 86, 103, 111, 142, 145, 147, 210, 211, 214,
        215, 229, 234, 238, 274, 284, 357, 362, 370, 429, 465
    ];
    for ix in tmpixs { parser.print_entry(ix); }
    println!("");
    */

    parser.resolve_sizes();

    return parser;
}
 

