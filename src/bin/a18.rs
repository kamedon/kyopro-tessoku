use std::cmp::min;

use proconio::input;

pub fn main() {
    input! { N: usize, S: i32, A: [i32;N] }
    run(&Args { s: S, a_list: A })
}

#[derive(Debug)]
struct Args {
    s: i32,
    a_list: Vec<i32>,
}

fn run(args: &Args) {
    println!("{:?}", args);
    let mut prev_ans_list: Vec<bool> = vec![false; (args.s  +1) as usize];
    prev_ans_list[0] = true;
    let ans = go(0, args.s, &args.a_list, &prev_ans_list);
    println!("{}", ans);
}

fn go(index: usize, s: i32, a_list: &Vec<i32>, prev_ans_list: &Vec<bool>) -> bool {
    println!("{}: {:?}", index, prev_ans_list);
    if index >= a_list.len() {
        return prev_ans_list[s as usize];
    }
    let mut ans: Vec<bool> = vec![];
    for si in 0..=s {
        if prev_ans_list[si as usize] {
            ans.push(true);
            continue;
        }

        let diff = si - a_list[index as usize];
        if diff >= 0 && prev_ans_list[diff as usize] {
            ans.push(true);
            continue;
        }
        ans.push(false);
    }

    return go(index + 1, s, a_list, &ans);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            s: 7,
            a_list: vec![2, 2, 3],
        });
    }
}