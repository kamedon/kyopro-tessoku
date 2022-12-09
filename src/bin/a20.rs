use std::cmp::{max, min};
use std::str::Chars;

use proconio::input;

pub fn main() {
    input! { S: String, T: String }
    run(&Args { s: S, t: T })
}

#[derive(Debug)]
struct Args {
    s: String,
    t: String,
}

fn run(args: &Args) {
    println!("{:?}", args);
    let scs: Vec<char> = args.s.chars().collect();
    let tcs: Vec<char> = args.t.chars().collect();
    let n = max(*&scs.len(), *&tcs.len()) + 1;
    let ans = vec![0 as u32; n];
    println!("{:?} , {:?}", scs, tcs);
    if *&scs.len() > *&tcs.len() {
        calc(0, &scs, &tcs, ans);
    } else {
        calc(0, &tcs, &scs, ans);
    }
}

fn calc(row: usize, scs: &Vec<char>, tcs: &Vec<char>, ans: Vec<u32>) -> Vec<u32> {
    if row == tcs.len() {
        return ans;
    }
    let t = tcs[row];
    let mut next: Vec<u32> = vec![];

    for i in 0..=scs.len() {
        match i {
            0 => {
                next.push(ans[0])
            }
            _ => {
                let s = scs[i - 1];
                let prev_next = next[i - 1];
                let prev_ans = ans[i];
                println!("{}: {}", t, s);
                if t == s {
                    next.push(max(max(prev_ans, prev_next), ans[i - 1] + 1));
                } else {
                    next.push(max(prev_ans, prev_next));
                }
            }
        }
    }

    println!("next: {:?}", next);
    return calc(row + 1, scs, tcs, next);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // run(&Args {
        //     t: "kyoto".to_string(),
        //     s: "tokyo".to_string(),
        // });
        run(&Args {
            t: "mynavi".to_string(),
            s: "monday".to_string(),
        });
    }
}