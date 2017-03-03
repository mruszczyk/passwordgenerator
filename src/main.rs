extern crate rand;


use std::io;
use rand::{thread_rng, Rng};


fn main() {
    println!("Generate a new password!");

    println!("Input parameters:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();

    let parsed = match trimmed.parse::<usize>() {
        Ok(i) => {
            i
        },
        Err(..) => {
            println!("This was not an integer: {}", trimmed);
            return;
        },
    };

    let pass = gen_pass(parsed);

    println!("{}", pass);


}

fn gen_pass(length: usize) -> String {
    let pass: String = thread_rng().gen_ascii_chars().take(length).collect();

    return pass;

}
