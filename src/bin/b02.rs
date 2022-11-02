use proconio::input;

pub fn main() {
    input! { a: i32, b:i32 }

    let mut ans = false;
    for n in a..=b {
        if 100 % n == 0 {
            ans = true;
            break;
        }
    }
    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}
