use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let input: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut total: i32 = 0;

    total += 3 * input[0];
    total += 2 * input[1];
    total += 1 * input[2];

    match total{
        0 | 1 => {println!("Copper")},
        2 => {println!("Estate or Copper")},
        3 | 4 => {println!("Estate or Silver")},
        5 => {println!("Duchy or Silver")},
        6 | 7 => {println!("Duchy or Gold")},
        _ => {println!("Province or Gold")},
    }
    

}
