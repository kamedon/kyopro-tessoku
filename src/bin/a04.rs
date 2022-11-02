use proconio::input;

pub fn main() {
    input! { n: u32}
    // println!("{:0>10b}", n)

    let mut p: [u32; 10] = [0; 10];
    let count = p.len();

    for i in 0..count {
        let pow = (2 as u32).pow(i as u32);
        p[i] = (n / pow % 2) as u32;
    }
    p.reverse();
    let ans = p.map(|v| v.to_string()).join("");
    println!("{}", ans)
}
