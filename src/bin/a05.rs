use proconio::input;

pub fn main() {
    input! {n: i32, k: i32}

    let mut ans = 0;
    'outer: for a in 1..=n {
        for b in 1..=n {
            let c: i32 = k - a - b;
            if c > 0 && c <= n {
                ans += 1;
            }
        }
    }
    println!("{}", ans)
}
