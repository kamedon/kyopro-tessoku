use std::cmp::{max, min};

use proconio::input;

pub fn main() {
    input! { N: usize, W: i32, I: [[i32;2];N] }
    run(&Args { w: W, items: I })
}

#[derive(Debug)]
struct Args {
    w: i32,
    items: Vec<Vec<i32>>,
}

fn run(args: &Args) {
    println!("{:?}", args);
    let mut ans = vec![i32::MIN; (args.w + 1) as usize];
    ans[0] = 0;
    let ans = go(0, args.w, &args.items, ans);
    println!("{:?}", ans);
}

fn go(index: usize, max_w: i32, items: &Vec<Vec<i32>>, prev_ans: Vec<i32>) -> Vec<i32> {
    if items.len() == index {
        return prev_ans;
    }

    let ans = (0..=max_w).map(|wi| {
        let prev = prev_ans[wi as usize];
        let mut max_v = max(i32::MIN, prev);
        let (w, v) = items.get(index).map(|item| (item[0], item[1])).unwrap();
        let dw = wi - w;
        if dw >= 0 {
            let vi = prev_ans[dw as usize] + v;
            max_v = max(max_v, vi);
        }
        max_v
    }).collect();
    return go(index + 1, max_w, items, ans);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            w: 7,
            items: vec![vec![3, 13], vec![3, 17], vec![5, 29], vec![1, 10]],
        });
    }
}