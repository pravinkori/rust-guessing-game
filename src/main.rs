// Import the io module from the standard library
use std::io;

// Define the entry point of the program
fn main() {
    // Print a message prompting the user to guess a number
    println!("Guess the number!");
    // Prompt the user to input their guess
    println!("Please input your guess.");

    // Declare a mutable variable named guess to store user input
    // Create a new empty String and bind it to the guess variable
    let mut guess = String::new();

    // Receive user input from the standard input (keyboard) and store it in the guess variable
    // The read_line method reads input from the standard input and appends it to the guess string
    // The &mut guess argument indicates that read_line should store the input in the guess variable
    // The expect method is called in case of an error during input reading
    // It displays the provided error message if an error occurs
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // Print the user's guess by using string interpolation
    // The {} is a placeholder for the value of the guess variable
    println!("You guessed: {}", guess);
}
