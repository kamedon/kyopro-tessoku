use proconio::input;

pub fn main() {
    input! { N: usize, A: [usize;N] }
    run(&Args { w: W, h: H, snows: S })
}

#[derive(Debug)]
struct Args {
    h: usize,
    w: usize,
    snows: Vec<Vec<usize>>,
}

fn run(args: &Args) {
    println!("{:?}", args);

    let mut map: Vec<Vec<i32>> = vec![vec![0; args.w + 1]; args.h + 1];
    let mut sumMap: Vec<Vec<i32>> = map.clone();

    for snow in args.snows.iter() {
        let a = snow[0];
        let b = snow[1];
        let c = snow[2];
        let d = snow[3];

        map[a][b] += 1;
        map[c + 1][b] -= 1;
        map[a][d + 1] -= 1;
        map[c + 1][d + 1] += 1;
    }

    for row in 0..args.h + 1 {
        for col in 0..args.w + 1 {
            if col == 0 {
                sumMap[row][col] = map[row][col]
            } else {
                sumMap[row][col] = sumMap[row][col - 1] + map[row][col]
            }
        }
    }

    for row in 0..args.h + 1 {
        for col in 0..args.w + 1 {
            if row == 0 {
                sumMap[row][col] = map[row][col]
            } else {
                sumMap[row][col] = sumMap[row - 1][col] + sumMap[row][col]
            }
        }
    }

    println!("{:?}", sumMap)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            w: 5,
            h: 5,
            snows: vec![vec![1, 1, 3, 3], vec![2, 2, 4, 4]],
        });
    }
}