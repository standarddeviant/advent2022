use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn run(fname: &str) {
    part1_simple(fname);
    part2_simple(fname);
}

fn part2_simple(fname: &str) {
    let scores: Vec<i32> = game_scores_2(fname);
    let sum: i32 =  scores.iter().sum();
    // println!("scores = {:?}", scores);
    println!("part2: sum(scores) = {}", sum);
}


fn game_scores_2(fname: &str) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    let rpclut = HashMap::from([
        ("A", RPC::Rock), ("B", RPC::Paper), ("C", RPC::Scissor),
        ("X", RPC::Rock), ("Y", RPC::Paper), ("Z", RPC::Scissor),
    ]);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                let keys: Vec<&str> = s.split(" ").collect();
                let theirs: RPC = rpclut[keys[0]];
                let mine: RPC = match theirs {
                    RPC::Rock => {
                        match keys[1] {
                            "X" => RPC::Scissor, 
                            "Y" => RPC::Rock, 
                            "Z" => RPC::Paper,
                            _ => RPC::Rock
                        }
                    },
                    RPC::Paper => {
                        match keys[1] {
                            "X" => RPC::Rock, 
                            "Y" => RPC::Paper, 
                            "Z" => RPC::Scissor,
                            _ => RPC::Rock
                        }
                    },
                    RPC::Scissor => {
                        match keys[1] {
                            "X" => RPC::Paper, 
                            "Y" => RPC::Scissor, 
                            "Z" => RPC::Rock,
                            _ => RPC::Rock
                        }
                    }
                }; // end let mine
                out.push(calc_score(theirs, mine));
           }
        }
    }
    return out;
}


fn part1_simple(fname: &str) {
    let scores: Vec<i32> = game_scores_1(fname);
    let sum: i32 =  scores.iter().sum();
    // println!("scores = {:?}", scores);
    println!("part1: sum(scores) = {}", sum);
}

#[derive(Copy, Clone, Debug)]
enum RPC {
    Rock,
    Paper,
    Scissor
}

fn game_scores_1(fname: &str) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    let rpclut = HashMap::from([
        ("A", RPC::Rock), ("B", RPC::Paper), ("C", RPC::Scissor),
        ("X", RPC::Rock), ("Y", RPC::Paper), ("Z", RPC::Scissor),
    ]);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                println!("{}", s);
                let keys: Vec<&str> = s.split(" ").collect();
                println!("{:?}", keys);
                println!("{:?}, {:?}, {:?}", 
                    rpclut[keys[0]], rpclut[keys[1]],
                    calc_score(rpclut[keys[0]], rpclut[keys[1]])
                );
 
                out.push(
                    calc_score(
                        rpclut[keys[0]],
                        rpclut[keys[1]]
                    )
                );
            }
        }
    }
    return out;
}

fn calc_score(theirs: RPC, mine: RPC) -> i32 {
    match mine {
        RPC::Rock => { 
            1 + match theirs {
                RPC::Rock   => 3,
                RPC::Paper  => 0,
                RPC::Scissor=> 6
            }
        },
        RPC::Paper => {
            2 + match theirs {
                RPC::Rock   => 6,
                RPC::Paper  => 3,
                RPC::Scissor=> 0
            }
        }
        RPC::Scissor => {
            3 + match theirs {
                RPC::Rock   => 0,
                RPC::Paper  => 6,
                RPC::Scissor=> 3
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}