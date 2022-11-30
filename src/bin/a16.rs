use std::cmp::min;

use proconio::input;

pub fn main() {
    input! { N: usize, A: [u32;N],B: [u32; N-1]}
    run(&Args { a_list: A, b_list: B })
}

#[derive(Debug)]
struct Args {
    a_list: Vec<u32>,
    b_list: Vec<u32>,
}

fn run(args: &Args) {
    println!("{:?}", args);
    let ans = go(1, 0, 0, &args.a_list, &args.b_list);
    print!("{}", ans);
}

fn go(index: usize, cost: u32, prev_cost: u32, a_list: &Vec<u32>, b_list: &Vec<u32>) -> u32 {
    println!("{}: {} , {}", index, cost, prev_cost);
    if index == a_list.len() + 1 {
        return cost;
    }

    if index == 1 {
        return go(index + 1, a_list[index - 1], cost, a_list, b_list);
    }


    let a_cost = cost + a_list[index - 1];
    let b_cost = prev_cost + b_list[index - 2];

    let current_cost = min(a_cost, b_cost);


    return go(index + 1, current_cost, cost, a_list, b_list);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            a_list: vec![2, 4, 1, 3],
            b_list: vec![5, 3, 3],
        });
    }
}