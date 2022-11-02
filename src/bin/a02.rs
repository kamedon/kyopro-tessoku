use proconio::input;

pub fn main() {
    input! { N: usize, X:i32, a: [i32; N]  }

    println!("{}", a.iter().any(|&v| v == X))
}
