use proconio::input;

#[derive(Debug, Copy, Clone)]
struct HitTotal {
    yes_total: u32,
    no_total: u32,
}

impl HitTotal {
    pub fn new() -> HitTotal {
        HitTotal {
            yes_total: 0,
            no_total: 0,
        }
    }
    pub fn from(yes: u32, no: u32) -> HitTotal {
        HitTotal {
            yes_total: yes,
            no_total: no,
        }
    }
}

pub fn main() {
    input! { N: u32, A: [u32;N], L: usize, R: usize}

    let mut total: Vec<HitTotal> = vec![];

    for (i, a) in A.iter().enumerate() {
        let mut prev_total = HitTotal::new();
        if i > 0 {
            prev_total = *total.get(i - 1).unwrap();
        }

        let yes = if *a == 1 { 1 } else { 0 };
        let no = if *a == 0 { 1 } else { 0 };

        let current_total = HitTotal::from(yes + prev_total.yes_total, no + prev_total.no_total);
        total.push(current_total)
    }

    let l = total.get(L).unwrap();
    let r = total.get(R).unwrap();

    let ans = HitTotal::from(r.yes_total - l.yes_total, r.no_total - l.no_total);
    if ans.yes_total > ans.no_total {
        println!("YES")
    } else if ans.no_total == ans.yes_total {
        println!("同数")
    } else {
        println!("NO")
    }

    println!()
}
