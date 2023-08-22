use std::io;

fn sum_of_digits(n: i32) -> i32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap() as i32).sum()
}

fn minimal_integer(l: i32, d: i32, x: i32) -> Option<i32> {
    for n in l..=d {
        if sum_of_digits(n) == x {
            return Some(n);
        }
    }
    None
}

fn maximal_integer(l: i32, d: i32, x: i32) -> Option<i32> {
    for n in (l..=d).rev() {
        if sum_of_digits(n) == x {
            return Some(n);
        }
    }
    None
}

fn main() {
    
    let mut l: i32 = 0;
    let mut d: i32 = 0;
    let mut x: i32 = 0;

    for i in 0..3 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error");
        match i {
            0 => {l = input.trim().parse().unwrap()},
            1 => {d = input.trim().parse().unwrap()},
            _ => {x = input.trim().parse().unwrap()},
        }
    }
    
    if let Some(min_n) = minimal_integer(l, d, x) {
        println!("{}", min_n);
    }

    if let Some(max_n) = maximal_integer(l, d, x) {
        println!("{}", max_n);
    }
}
