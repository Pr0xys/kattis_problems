use std::io;
use std::collections::HashMap;

fn main() {
    let numbers: HashMap<char,i32> = HashMap::from([
        ('a', 2),
        ('b', 22),
        ('c', 222),
        ('d', 3),
        ('e', 33),
        ('f', 333),
        ('g', 4),
        ('h', 44),
        ('i', 444),
        ('j', 5),
        ('k', 55),
        ('l', 555),
        ('m', 6),
        ('n', 66),
        ('o', 666),
        ('p', 7),
        ('q', 77),
        ('r', 777),
        ('s', 7777),
        ('t', 8),
        ('u', 88),
        ('v', 888),
        ('w', 9),
        ('x', 99),
        ('y', 999),
        ('z', 9999),
        (' ', 0),
    ]);

    let mut n_input = String::new();
    io::stdin().read_line(&mut n_input).expect("Failed to read line");
    
    let n: i32 = n_input.trim().parse().expect("Please enter a number");

    for p in 1..=n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut output = String::new();
        
        let mut prev_string = String::new();
        

        for x in input.trim_end_matches('\n').to_lowercase().chars() {
            
            let current_string = numbers.get(&x).unwrap();
            let current_string = format!("{}",current_string);

            if current_string.contains(&prev_string) || prev_string.contains(&current_string){
                output.push_str(" ");
            }

            output.push_str(&current_string);
            prev_string = current_string;
            
        
        }
        
        println!("Case #{}:{}", p, output);
    }   
}
