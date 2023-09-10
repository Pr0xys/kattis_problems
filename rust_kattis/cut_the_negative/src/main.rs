use std::io;
use std::convert::TryInto;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("input error");

    let n: i32 = input.trim().parse().unwrap();
    let mut total: i32 = 0;
    let mut output_vector: Vec<Vec<i32>> = Vec::new();
    
    for i in 1..n + 1 {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("input error");
            
        let inp_vec: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
            
        let mut visited_indices = Vec::new();

        for j in &inp_vec {
            if *j > 0 {
                total += 1;
                
                let mut aux_vec: Vec<i32> = Vec::new();
                
                let add_this = inp_vec.iter().enumerate().find(|(pos, x)| !visited_indices.contains(pos) && **x == *j)
                    .map(|(pos, _)| pos);
                    
                if let Some(pos) = add_this {
                    visited_indices.push(pos);
                }

                aux_vec.push(i);
                
                match add_this {
                    Some(pos) => match (pos + 1).try_into() {
                        Ok(pos_i32) => aux_vec.push(pos_i32),
                        Err(_) => println!("Conversion failed, value out of range"),
                    },
                    None => aux_vec.push(-1),
                }
                
                aux_vec.push(*j);
                output_vector.push(aux_vec); 
            }
        }
    }

    println!("{}", total);
    
    for vec in output_vector.iter() {
        let vec_as_str: Vec<String> = vec.iter().map(|&num| num.to_string()).collect();
        let formatted_output = vec_as_str.join(" ");
        println!("{}", formatted_output);
    }
}