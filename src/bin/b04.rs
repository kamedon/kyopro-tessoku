use proconio::input;

pub fn main() {
    input! { n: String}
    let mut nums: Vec<u32> = n.chars().map(|v| v.to_digit(2).unwrap()).collect();
    println!("{:?}", nums);
    nums.reverse();

    let mut ans = 0;
    for i in 0..nums.len() {
        ans += (2 as u32).pow(i as u32) * nums.get(i).unwrap()
    }

    println!("{}", ans)
}
