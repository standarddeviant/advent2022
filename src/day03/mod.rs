use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use crate::utils::read_lines;

pub fn run(fname: &str) {
    let cvec = compartment_samesies(fname);
    //println!("{:?}", cvec);
    let sum1: i32 = cvec.iter().map(score_char).sum();
    println!("part1 sum = {}", sum1);

    let bvec = badge_samesies(fname);
    let sum2: i32 = bvec.iter().map(score_char).sum();
    println!("part2 sum = {}", sum2);
}

fn badge_samesies(fname: &str) -> Vec<char> {
    let mut out: Vec<char> = vec![];
    if let Ok(lines_iter) = read_lines(fname) {
        // let lines: Vec<Result<String, std::io::Error>> = lines_iter.collect();
        let lines: Vec<_> = lines_iter.collect();
        for chunk3 in lines.chunks(3) {
            let mut hv: Vec<HashSet<char, RandomState>> = vec![];
            for ix in 0..3 {
                hv.push(
                    HashSet::from_iter(
                        chunk3[ix].as_ref().expect("whoops....").chars()
                    )
                );
            }
            // println!("hv = {:?}", hv);
            let s01: String = hv[0].intersection(&(hv[1])).into_iter().collect();
            let s12: String = hv[1].intersection(&(hv[2])).into_iter().collect();
            //println!("s01 = {:?}", s01);
            //println!("s12 = {:?}", s12);

            let hx1: HashSet<char, RandomState> = HashSet::from_iter(s01.chars());
            let hx2: HashSet<char, RandomState> = HashSet::from_iter(s12.chars());
            let badge: char = *(hx1.intersection(&hx2).take(1).next().expect("le fin..."));

            // println!("badge = {}", badge);

            out.push(badge);
            
           /*

            let vecs3 = chunk3.as_ref().expect("Error creating group of 3");

            let mut intersect_result: Vec<char> = chunk3[0].into_iter();
            for temp_vec in chunk3 {
                let unique_a: HashSet<i32> = temp_vec.into_iter().collect();
                intersect_result = unique_a
                    .intersection(&intersect_result.into_iter().collect())
                    .map(|i| *i)
                    .collect::<Vec<_>>();
            }
            intersect_result
            */
        }

        /*
        let hsets: Vec<HashSet<char>> =
            chunk3.iter()
            .map(|x| {
                HashSet::from_iter(tmp1.chars())
            }).into_iter()
            .collect();
        println!("hsets = {:?}", hsets);

        let is01: Vec<&char> = hsets[0].intersection(&hsets[1]).collect();
        println!("is01 = {:?}", is01);
        let h01: HashSet<&char> = HashSet::from_iter(is01);
        println!("h01 = {:?}", h01);
        let h012: HashSet<&char> = (&hsets[2]);
        */
    }
    return out;
}
       
fn compartment_samesies(fname: &str) -> Vec<char> {
    let mut out: Vec<char> = vec![];
    if let Ok(lines) = read_lines(fname) {
        for line in lines {
            if let Ok(s) = line {
                let half: usize = s.len() / 2;
                let (comp1, comp2) = (&s[..half], &s[half..]);
                // println!("c1 = {}, c2 = {}", c1, c2);
                let s1: HashSet<char> = HashSet::from_iter(comp1.chars());
                let s2: HashSet<char> = HashSet::from_iter(comp2.chars());
                let inter: Vec<&char> = s1.intersection(&s2).collect();
                // println!("{:?}", inter);
                assert_eq!(inter.len(), 1);
                out.push(*inter[0]);
            }
        }
    }
    return out;
}

fn score_char(c: &char) -> i32 {
    let r = u8::try_from(*c);
    let u8score: u8 = match r {
        Ok(x) => {
            if x > 96 {
                x - 96
            }
            else {
                x - 64 + 26
            }
        },
        Err(_e) => 0
    };
    let score = u8score as i32;
    return score;
}
