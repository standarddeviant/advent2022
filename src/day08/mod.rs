// use std::collections::hash_map::RandomState;
// use std::collections::HashSet;
use crate::utils::read_lines;
use ndarray::prelude::*;
use ndarray_stats::QuantileExt;


pub fn run(fname: &str) {
    let a: Array<i32, Ix2> = input_parse(fname);
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
                        if vizbool {
                            println!("[true] {this_val} @ [{ixrow}, {ixcol}]");
                            println!("    north {} = {:?}", nbool, nmax);
                            println!("    south {} = {:?}", sbool, smax);
                            println!("     east {} = {:?}", ebool, emax);
                            println!("     west {} = {:?}", wbool, wmax);
                            println!("     west = {west:?}");
                        }
                        if vizbool {1} else {0}
                    };
                    *bref = viz as u32;
                }
            }
        }
    }

    println!("b = {:?}", b);
    println!("b.sum() = {:?}", b.sum());
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
                // let i32arr1: ArrayBase<ndarray::OwnedRepr<i32>, Dim<[usize; 1]>> = ndarray::arr1(&i32vec);
                // vv.push(i32vec);
                // vv.push(i32arr1);
                //println!("{:?}", i32vec);
                //println!("{:?}", ok_line.trim().split(""));
                // println!("{ok_line}");
            }
        }
    }
    // let a: ArrayBase<ndarray::OwnedRepr<i32>, Dim<[usize; 2]>> = 
    let a: Array<i32, Ix2> =
        Array::<i32, _>::from_vec(vv)
        .into_shape((ncols, nrows))
        .unwrap();
    println!("{:?}", a);
    return a;

}
       
