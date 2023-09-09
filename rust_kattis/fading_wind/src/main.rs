use std::io;
use std::cmp::max;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    let input: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x
             .parse()
             .unwrap()
             )
        .collect();
    
    let mut h: i32 = input[0];
    let k: i32 = input[1];
    let mut v: i32 = input[2];
    let mut s: i32 = input[3];
    let mut distance: i32 = 0;

    while h > 0 {
        v += s;
        v -= max(1, v / 10);
        
        if v >= k {
            h += 1;
        }
        if v > 0 && v < k {
            h -= 1;
        }
        

        distance += v;
        
    
        if h <= 0 {
            v = 0;
            s = 0;
            h = 0;
        }
        
        if v <= 0 {
            h = 0;
            v = 0;
        }
    
        if s > 0 {
            s -= 1;
        }
    }

    println!("{}", distance);
}

