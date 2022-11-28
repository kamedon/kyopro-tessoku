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
    let len = args.list.len();
    let mut count = 0;
    for i in 0..len {
        count += calc(args.k, &args.list[i..]);
    }
    println!("{}", count);
}

fn calc(k: u32, list: &[u32]) -> u32 {
    let len = list.len();
    if len < 2 {
        return 0;
    }
    let mut count: u32 = 0;

    for i in 1..len {
        let diff = list[i] - list[0];
        if diff <= k {
            count += 1;
            println!("{} {} {}", list[i], list[0], diff);
        } else {
            break;
        }
    }
    println!("==> {}", count);
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            k: 10,
            list: vec![11, 12, 16, 22, 27, 28, 31],
        });
    }
}