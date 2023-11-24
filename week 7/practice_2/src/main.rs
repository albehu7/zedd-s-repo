use std::io;

fn checker(){

    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let :: char = input.trim().parse().expect("Invalid input");

    if >= '0' &&  <= '9'
    {
        println!("Character '{}' is a digit",);
    }
    else
    {
        println!("Character '{}' is not a digit",);
        
    }
    
}

fn main() {
    // calling function
    println!("Welcome! This program checks wether a character variable
        contains a digit or not");
    checker()
}