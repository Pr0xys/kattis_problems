use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error");
    
    let this_list: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x
             .parse()
             .unwrap()
             )
        .collect();
    
    let mut counter:i32 = 0;

    for _ in 0 .. this_list[0]{
        input
            .clear();
        
        io::stdin()
            .read_line(&mut input)
            .expect("error");
        
        counter += input
            .trim()
            .parse::<i32>()
            .unwrap();
 
    }
    
    match counter <= this_list[1] {
        true => println!("Jebb"),
        false => println!("Neibb"),
    }

}
