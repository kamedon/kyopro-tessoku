use std::process::id;
use std::ptr::null;
use std::collections::HashSet;
use std::collections::HashMap;

use proconio::input;

pub fn main() {
    input! { N: usize, A: [u32;N]}
    run(&Args { a_list: A })
}

#[derive(Debug)]
struct Args {
    a_list: Vec<u32>,
}

fn run(args: &Args) {
    let b: HashSet<u32> = args.a_list.clone().into_iter().collect();
    let mut b: Vec<u32> = b.into_iter().collect();
    b.sort();
    let mut i: u32 = 0;
    let mut map: HashMap<u32, u32> = HashMap::new();
    b.iter().for_each(|v| {
        i += 1;
        map.insert(*v, i);
    });
    let list: Vec<u32> = args.a_list.iter().map(|v| *map.get(v).unwrap()).collect();
    println!("{:?}", list)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        run(&Args {
            a_list: vec![46, 80, 11, 77, 46]
        });
    }
}