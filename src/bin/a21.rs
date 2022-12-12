use std::cmp::{max, min};
use std::str::Chars;

use proconio::input;

pub fn main() {
    input! { N:usize ,BS: [[i32;2];N]}
    run(&Args { bs: BS })
}

#[derive(Debug)]
struct Args {
    bs: Vec<Vec<i32>>,
}

fn run(args: &Args) {
    println!("{:?}", args);
    let ans = calc(1, args.bs.len(), &args.bs, 0);
    println!("{}", ans);
}

fn calc(left: usize, right: usize, bs: &Vec<Vec<i32>>, total: i32) -> i32 {
    println!("({},{}): {}", left, right, total);
    if left == right {
        return total;
    }

    // Left
    let l = bs.get(left - 1).unwrap();
    let (lp, la) = (l[0] as usize, l[1]);
    let lt = if lp <= right && lp >= left {
        total + la
    } else {
        total
    };

    let l_ans = calc(left + 1, right, bs, lt);

    // Right
    let r = bs.get(right - 1).unwrap();
    let (rp, ra) = (r[0] as usize, r[1]);
    let rt = if rp <= right && rp >= left {
        total + ra
    } else {
        total
    };

    let r_ans = calc(left, right - 1, bs, rt);

    return max(l_ans, r_ans);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            bs: vec![vec![4, 20], vec![3, 20], vec![2, 40], vec![1, 10]]
        });
    }
}