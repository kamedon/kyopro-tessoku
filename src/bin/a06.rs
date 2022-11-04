use proconio::input;

pub fn main() {
    input! { N: usize, Q: usize, A: [i32; N] , L: [[usize;2];Q] }
    //
    let mut sums: Vec<i32> = vec![];
    //
    for i in 0..N {
        let ai = A.get(i).unwrap();
        let mut prev_sum = 0;
        if i > 0 {
            prev_sum = *sums.get(i - 1).unwrap();
        }
        let sum = prev_sum + ai;
        sums.push(sum);
    }

    for l in L {
        let start = l.get(0).unwrap();
        let end = l.get(1).unwrap();

        let s = sums.get(*start).unwrap();
        let e = sums.get(*end).unwrap();

        let ans = e - s;
        println!("{:?}", ans);
    }

    // println!("{:?}", sums);
    // println!("{:?}", L);
}
