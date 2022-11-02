use proconio::input;

pub fn main() {
    input! { N: usize, P: [i32; N]   }

    let mut ans = false;

    'outer: for i in 0..N {
        for j in 1..N {
            for k in 2..N {
                if i == j || i == k || j == k {
                    break;
                }
                let a = P.get(i).unwrap();
                let b = P.get(j).unwrap();
                let c = P.get(k).unwrap();

                if a + b + c == 1000 {
                    ans = true;
                    break 'outer;
                }
            }
        }
    }

    if ans {
        println!("YES")
    } else {
        println!("NO")
    }
}
