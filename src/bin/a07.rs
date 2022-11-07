use proconio::input;

pub fn main() {
    input! { D: usize, N: usize, A : [[usize;2];N] }
    //
    let mut sums: Vec<i32> = vec![0; D + 1];
    let mut total: Vec<i32> = vec![0; D + 1];
    //
    for a in A {
        let l = *a.get(0).unwrap();
        let r = *a.get(1).unwrap();

        let sum_l = *sums.get(l).unwrap();
        sums[l] = sum_l + 1;

        let sum_r = *sums.get(r + 1).unwrap();
        sums[r + 1] = sum_r - 1;
    }

    let mut sum = 0;
    for (i, v) in sums.iter().enumerate() {
        sum += v;
        total[i] = sum;
    }

    println!("{:?}", sums);
    println!("{:?}", total);
}
