use proconio::input;

pub fn main() {
    input! { N: usize,  X: i32, L: [i32;N] }
    run(&Args { x: X, list: L })
}

#[derive(Debug)]
struct Args {
    x: i32,
    list: Vec<i32>,
}

fn run(args: &Args) {
    println!("{:?}", args.list);
    let index = compare(args.x, &args.list, 0);
    println!("{}", index + 1)
}

fn compare(x: i32, list: &[i32], index: usize) -> usize {
    // 11 / 2 = 5
    // 5, 6
    let count = list.len();
    if count == 1 {
        return index;
    }
    let half_index = count / 2;
    return if x < list[half_index] {
        println!("{}: {:?}", index, &list[..half_index]);
        compare(x, &list[..half_index], index)
        //
    } else {
        println!("{}: {:?}", index + half_index, &list[half_index..]);
        compare(x, &list[half_index..], index + half_index)
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            x: 47,
            list: vec![11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67],
        });

        run(&Args {
            x: 80,
            list: vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100],
        });
    }
}