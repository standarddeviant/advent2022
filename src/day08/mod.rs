// use std::collections::hash_map::RandomState;
// use std::collections::HashSet;
use crate::utils::read_lines;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;


pub fn run(fname: &str) {
    let a: Array<i32, Ix2> = input_parse(fname);
    part1(&a);
    part2(&a);
}

fn part2(a: &Array<i32, Ix2>) {
    let (ncols, nrows) = a.dim();
    // let mut s: Array< u32, Ix2> = Array::zeros([ncols, nrows]);
    let (mut max_score, mut max_row, mut max_col) = (0, 0, 0);
    for ixcol in 1..ncols {
        for ixrow in 1..nrows {
            if let Some(this_val) = a.get((ixrow, ixcol)) {
                /*
                */
                // let mut score;
                fn score_part(val: &i32, av: ArrayView1<i32>) -> i32 {
                    let mut out = 0;
                    for tmpval in av.iter() {
                        out += 1;
                        if tmpval >= val {
                            break;
                        }
                    }
                    return out;
                }

                let north: ArrayView1<_> = a.slice(s![0..ixrow     ; -1,    ixcol     ]);
                let south: ArrayView1<_> = a.slice(s![   ixrow+1..     ,    ixcol     ]);
                let  east: ArrayView1<_> = a.slice(s![   ixrow         ,    ixcol+1.. ]);
                let  west: ArrayView1<_> = a.slice(s![   ixrow         , 0..ixcol ; -1]);

                // println!("[{ixrow}, {ixcol}]:\n    north={north}\n    south={south}\n    escore={east}\n    west={west}");
                // if ixrow > 3 {
                //     pass return;
                // };
                let (nscore, sscore, escore, wscore) = (
                    score_part(this_val, north),
                    score_part(this_val, south),
                    score_part(this_val,  east),
                    score_part(this_val,  west)
                );
                // println!("[{ixrow}, {ixcol}]: nscore={nscore}, sscore={sscore}, escore={escore}, wscore={wscore}");
                let score = nscore * sscore * escore * wscore;
                if score >= max_score {
                    max_score = score;
                    max_row = ixrow;
                    max_col = ixcol;
                }
            }
        }
    }

    println!("part2: max_score = {max_score} @ [r={max_row}, c={max_col}]");
}

fn part1(a: &Array<i32, Ix2>) {
    let (ncols, nrows) = a.dim();
    let mut b: Array< u32, Ix2> = Array::zeros([ncols, nrows]);
    //let ncols = a.shape().get(0).unwrap();
    //let nrows = a.shape().get(0).unwrap();
    for ixcol in 0..ncols {
        for ixrow in 0..nrows {
            if let Some(bref) = b.get_mut((ixcol, ixrow)) {
                if 0==ixcol || 0==ixrow || ixcol==ncols-1 || ixrow==nrows-1 {
                    // println!("edge @ [{ixcol}, {ixrow}]");
                    *bref = 1;
                    continue;
                }
                if let Some(this_val) = a.get((ixrow, ixcol)) {
                    let viz: i32 = {
                        let north: ArrayView1<_> = a.slice(s![0..ixrow   ,    ixcol   ]);
                        let south: ArrayView1<_> = a.slice(s![ixrow+1..  ,    ixcol   ]);
                        let  east: ArrayView1<_> = a.slice(s![   ixrow   , ixcol+1..  ]);
                        let  west: ArrayView1<_> = a.slice(s![   ixrow   , 0..ixcol   ]);
                        // this > north.min
                        let (nmax, smax, emax, wmax) = (
                            north.max().unwrap(), south.max().unwrap(), 
                             east.max().unwrap(),  west.max().unwrap()
                        );
                        let (nbool, sbool, ebool, wbool) = (
                            this_val > nmax, this_val > smax,
                            this_val > emax, this_val > wmax
                        );
                        let vizbool = nbool || sbool || ebool || wbool;
                        /*
                        if vizbool {
                            println!("[true] {this_val} @ [{ixrow}, {ixcol}]");
                            println!("    north {} = {:?}", nbool, nmax);
                            println!("    south {} = {:?}", sbool, smax);
                            println!("     east {} = {:?}", ebool, emax);
                            println!("     west {} = {:?}", wbool, wmax);
                            println!("     west = {west:?}");
                        } */
                        if vizbool {1} else {0}
                    };
                    *bref = viz as u32;
                }
            }
        }
    }

    // println!("b = {:?}", b);
    println!("part1: b.sum() = {:?}", b.sum());
}



fn input_parse(fname: &str) -> Array<i32, Ix2>  { // <ndarray::OwnedRepr<i32>, Dim<[usize; 2]>> {
    // let mut vv: Vec<Vec<i32>> = vec![];
    let mut vv: Vec<i32> = vec![];
    let (mut ncols, mut nrows) = (0, 0);
    if let Ok(lines) = read_lines(fname){
        for line in lines {
            if let Ok(ok_line) = line {
                let i32vec: Vec<i32> =
                    ok_line.chars().into_iter().map(
                        |c| c as i32 - 0x30
                    ).collect();
                ncols += 1;
                nrows = i32vec.len();
                for i32val in i32vec {
                    vv.push(i32val);
                }
           }
        }
    }
    // let a: ArrayBase<ndarray::OwnedRepr<i32>, Dim<[usize; 2]>> = 
    let a: Array<i32, Ix2> =
        Array::<i32, _>::from_vec(vv)
        .into_shape((ncols, nrows))
        .unwrap();
    // println!("{:?}", a);
    return a;

}
       
