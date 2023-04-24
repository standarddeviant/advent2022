use std::collections::HashMap;
use crate::utils::read_lines;

#[derive(Copy, Clone, Debug)]
enum RPC {
    Rock,
    Paper,
    Scissor
}

pub fn run(fname: &str) {
    let pairs = abcxyz_pairs(fname);

    let scores1: Vec<i32> = game_scores_1(&pairs);
    let sum1: i32 =  scores1.iter().sum();
    println!("part2: sum(scores) = {}", sum1);

    let scores2: Vec<i32> = game_scores_2(&pairs);
    let sum2: i32 =  scores2.iter().sum();
    println!("part2: sum(scores) = {}", sum2);
}

fn game_scores_2(pairs: &Vec<(char, char)>) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    let theirs_lut= HashMap::from([
        ('A', RPC::Rock), ('B', RPC::Paper), ('C', RPC::Scissor)
    ]);
    let mine_lut= HashMap::from([
        (('A', 'X'), RPC::Scissor), // rock ,   lose -> scissor
        (('A', 'Y'), RPC::Rock   ), // rock ,   draw -> rock
        (('A', 'Z'), RPC::Paper  ), // rock ,   win  -> paper
        (('B', 'X'), RPC::Rock   ), // paper,   lose -> rock
        (('B', 'Y'), RPC::Paper  ), // paper,   draw -> paper
        (('B', 'Z'), RPC::Scissor), // paper,   win  -> scissor
        (('C', 'X'), RPC::Paper  ), // scissor, lose -> paper
        (('C', 'Y'), RPC::Scissor), // scissor, draw -> scissor
        (('C', 'Z'), RPC::Rock   ), // scissor, win  -> rock
    ]);
    for pair in pairs {
        out.push(calc_score(
            theirs_lut[&pair.0],
            mine_lut[&pair]
        ))
    }
   return out;
}


fn game_scores_1(pairs: &Vec<(char, char)>) -> Vec<i32> {
    let mut out: Vec<i32> = vec![];
    let theirs_lut= HashMap::from([
        ('A', RPC::Rock), ('B', RPC::Paper), ('C', RPC::Scissor)
    ]);
    let mine_lut= HashMap::from([
        ('X', RPC::Rock), ('Y', RPC::Paper), ('Z', RPC::Scissor)
    ]);
    for pair in pairs {
        out.push(calc_score(
            theirs_lut[&pair.0],
            mine_lut[&pair.1]
        ))
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

fn abcxyz_pairs(fname: &str) -> Vec<(char, char)> {
    let mut out: Vec<(char, char)> = vec![];

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(fname) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(s) = line {
                let tokens: Vec<&str> = s.split(' ').collect();
                let keys: (char, char) = (
                    tokens[0].chars().nth(0).unwrap(),
                    tokens[1].chars().nth(0).unwrap()
                );
                out.push(keys);
           }
        }
    }
    return out;
}
