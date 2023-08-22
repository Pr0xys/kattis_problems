use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error");
    let check_list: Vec<i32> = vec![4,3,2,7,6,5,4,3,2,1];
    let mut counter: usize = 0;
    let mut output: i32 = 0;
    for ch in input.trim().chars(){
        if ch != '-' {
            let n: i32 = (ch.to_string()).parse::<i32>().unwrap();
            let x: i32 = check_list[counter];
            output += n * x;
            counter += 1; 
        }
    }
    if output%11 == 0 {
        println!("1");
    } else  {
        println!("0");
    }
}
