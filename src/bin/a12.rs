use std::process::id;
use std::ptr::null;

use proconio::input;

pub fn main() {
    input! { N: usize,  K: u32, A: [u32;N] }
    run(&Args { k: K, list: A })
}

#[derive(Debug)]
struct Args {
    k: u32,
    list: Vec<u32>,
}

fn run(args: &Args) {
    let N: u32 = 100_000;
    let result = calc(args.k, &args.list, N / 2, Some(N));
    println!("{}", result)
}

fn calc(k: u32, vec: &[u32], current_time: u32, done_time: Option<u32>) -> u32 {
    println!("{}: {:?}", current_time, done_time);
    if done_time == Some(current_time) {
        return current_time;
    }
    let sum = vec.iter().fold(0, |sum, v| sum + (current_time / *v));
    return if k <= sum {
        let next = current_time / 2;
        calc(k, vec, next, Some(current_time))
    } else {
        let next = match done_time {
            None => { current_time + (current_time / 2) }
            Some(v) => {
                let s = (v - current_time);
                current_time + (s / 2) + (s % 2)
            }
        };
        calc(k, vec, next, done_time)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            k: 10,
            list: vec![1, 2, 3, 4],
        });
    }
}