use proconio::input;

pub fn main() {
    input! { H: usize, W: usize,  X: [[usize;W];H],Qc:usize,Q: [[usize;4];Qc] }
    run(&Args { q: Q, x: X })
}

#[derive(Debug)]
struct Args {
    x: Vec<Vec<usize>>,
    q: Vec<Vec<usize>>,
}

fn run(args: &Args) {
    println!("{:?}", args);

    //
    let mut col_sums: Vec<Vec<usize>> = vec![];
    let mut row_sums: Vec<Vec<usize>> = vec![];
    let col_count = args.x.len();
    let row_count = args.x.first().unwrap().len();

    for x in args.x.iter() {
        let mut col = 0;
        let mut col_sum: Vec<usize> = vec![];
        for v in x {
            col += v;
            col_sum.push(col);
        }
        col_sums.push(col_sum);
    }

    for c in 0..col_count {
        let mut row = 0;
        let mut row_sum: Vec<usize> = vec![];
        for r in 0..row_count {
            let v = col_sums.get(r).unwrap().get(c).unwrap();
            row += v;
            row_sum.push(row);
        }
        row_sums.push(row_sum);
    }

    for q in args.q.iter() {
        let a = *q.get(0).unwrap() as i32 - 2;
        let b = *q.get(1).unwrap() as i32 - 2;
        let c = *q.get(2).unwrap() as i32 - 1;
        let d = *q.get(3).unwrap() as i32 - 1;

        let lt = if a < 0 || b < 0 {
            0
        } else {
            *row_sums.get(b as usize).unwrap().get(a as usize).unwrap()
        };

        let lb = if a < 0 {
            0
        } else {
            *row_sums.get(d as usize).unwrap().get(a as usize).unwrap()
        };
        let rt = if b < 0 {
            0
        } else {
            *row_sums.get(b as usize).unwrap().get(c as usize).unwrap()
        };
        let rb = *row_sums.get(d as usize).unwrap().get(c as usize ).unwrap();

        println!("{}", lt + rb - rt - lb)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            x: vec![vec![2, 0, 0, 5, 1], vec![1, 0, 3, 0, 0], vec![0, 8, 5, 0, 2], vec![4, 1, 0, 0, 6], vec![0, 9, 2, 7, 0]],
            q: vec![vec![2, 2, 4, 5], vec![1, 1, 5, 5]],
        });
    }
}