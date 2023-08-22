use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let number: i32 = input.trim().parse().unwrap();
    let mut total: i32 = 0;
    for _i in 0..number{
        input.clear();
        io::stdin().read_line(&mut input).expect("error");
        let new_number: i32 = input.trim().parse().unwrap();
        total += new_number; 
    }
    println!("{}", total - (number - 1));    
}
