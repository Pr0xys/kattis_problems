use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error!");
    let input: i32 = input.trim().parse().unwrap();

    for i in 1 .. input+1 {
        let mut new_input = String::new();
        io::stdin().read_line(&mut new_input).expect("error!");
        new_input.clear();
        io::stdin().read_line(&mut new_input).expect("error!");
        let first_list: Vec<_> = new_input.trim().split_whitespace().collect();
        let mut output_list: Vec<_> = Vec::new();
        for i in 0..first_list.len(){
            if output_list.contains(&first_list[i]){
                output_list.retain(|x| x != &first_list[i]);
            } else {
                output_list.push(&first_list[i]);
            }
        }
        
        let mut output = String::new();
        for i in 0 .. output_list.len(){
            output += output_list[i];
        }
        
        println!("Case #{}: {}", i,  output);
    }
}
