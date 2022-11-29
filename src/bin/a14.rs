use std::process::id;
use std::ptr::null;

use proconio::input;

pub fn main() {
    input! { N: usize,  K: u32, A: [u32;N] ,B: [u32;N], C: [u32;N], D: [u32;N]}
    run(&Args { k: K, a_box: A, b_box: B, c_box: C, d_box: D })
}

#[derive(Debug)]
struct Args {
    k: u32,
    a_box: Vec<u32>,
    b_box: Vec<u32>,
    c_box: Vec<u32>,
    d_box: Vec<u32>,
}

fn run(args: &Args) {
    let abSum = sum(&args.a_box, &args.b_box);
    let cdSum = sum(&args.c_box, &args.d_box);
    let count = abSum.len();
    let k: i32 = args.k as i32;
    println!("{:?}", abSum);
    println!("{:?}", cdSum);

    for i in 0..count {
        let v: i32 = (abSum[i] as i32) - k;
        if v <= 0 {
            continue;
        }
        let v = v as u32;
        println!("{:?}", v);

        let hit = cdSum.iter().find(|cdV| v == **cdV);
        match hit {
            None => {}
            Some(_) => {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}


fn sum(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut vec: Vec<u32> = vec![];
    a.iter().for_each(|av| b.iter().for_each(|bv| vec.push(*av + *bv)));
    vec.sort();
    return vec;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            k: 10,
            a_box: vec![3, 9, 17],
            b_box: vec![4, 7, 9],
            c_box: vec![10, 20, 30],
            d_box: vec![1, 2, 3],
        });
    }
}