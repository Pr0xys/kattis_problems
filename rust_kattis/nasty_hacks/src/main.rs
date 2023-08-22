use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let n: i32 = input.trim().parse().unwrap();

    for _ in 0 .. n{
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let sum_it: Vec<i32> = input.trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if sum_it[0] + sum_it[2] < sum_it[1] {
            println!("advertise");
        }
        else if  sum_it[0] + sum_it[2] == sum_it[1]{ 
            println!("does not matter");
        }
        else{
            println!("do not advertise");
        }
    }
    
}
