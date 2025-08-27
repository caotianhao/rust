use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter the word:");

    io::stdin().read_line(&mut input).unwrap();

    println!("{}", input);
}
