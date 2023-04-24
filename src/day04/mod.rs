
use crate::utils::read_lines;

struct Range{
    a: i32,
    b: i32
}
impl Range {
    /* 
    pub fn new(a: i32, b: i32) -> Self {
        return Self { a: a, b: b };
    }
    */
    pub fn from_string(s: &str) -> Self {
        let tmp: Vec<i32> = 
            s.split("-")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        assert_eq!(tmp.len(), 2);
        return Range { a: tmp[0], b: tmp[1] };
    }
    pub fn full_overlap(&self, othr: &Range) -> i32 {
        return {
            if      (self.a <= othr.a) & (othr.b <= self.b) { 1 }
            else if (othr.a <= self.a) & (self.b <= othr.b) { 1 }
            else                                            { 0 }
        }
    }
    pub fn part_overlap(&self, othr: &Range) -> i32 {
        return {
            if      (self.a <= othr.b) & (othr.a <= self.b) { 1 }
            else if (othr.a <= self.b) & (self.a <= othr.b) { 1 }
            else                                            { 0 }
        }
    }
}

pub fn run(fname: &str) {
    let ranges = get_ranges(fname);
    let sum1: i32 = 
        ranges
        .iter().map(|x| {
            let (r1, r2) = x;
            r1.full_overlap(r2)
        })
        .sum();
    println!("part1: count = {}", sum1);
    let sum2: i32 = 
        ranges
        .iter().map(|x| {
            let (r1, r2) = x;
            r1.part_overlap(r2)
        })
        .sum();
    println!("part2: count = {}", sum2);
}

fn get_ranges(fname: &str) -> Vec<(Range, Range)> {
    let mut out: Vec<(Range, Range)> = vec![];
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                let rngs: Vec<&str>= s.split(",").collect();
                assert_eq!(rngs.len(), 2);
                let (s1, s2) = (rngs[0], rngs[1]);
                let rtup:(Range, Range) = (Range::from_string(s1), Range::from_string(s2));
                out.push(rtup);
           }
        } /* end for line in lines */
    } /* end let Ok(lines) = readlines(...) */
    //return count;
    return out;
}


