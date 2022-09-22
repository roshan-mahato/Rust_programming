use std::io;
// use std::rand;
// use compare::{Compare, natural};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Enter the number: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");

    let guessed_number = guess.trim().parse::<i32>().unwrap();

    println!("You guessed: {guess}");

    let mut rng = rand::thread_rng();
    let n1: i32 = rng.gen_range(0..10);

    println!("The number: {n1}");

    // let cmp = natural();
    if guessed_number == n1
    {
        println!("You Win!!!");
    }
    else
    {
        println!("You Lose!!!");
    }


}