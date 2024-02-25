// Import the io module from the standard library
use std::io;
// Import the Rng trait from the rand crate to generate random numbers
use rand::Rng;
use std::cmp::Ordering;

// Define the entry point of the program
fn main() {
    // Print a message prompting the user to guess a number
    println!("Guess the number!");

    /*
    Generate a random number within the range of 1 to 100 (inclusive) 
    - rand::thread_rng() creates a random number generator tied to the current thread 
    - gen_range() method generates a random number within the specified range
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop continuously until the user guesses the correct number
    loop {
        // Prompt the user to input their guess
        println!("Please input your guess.");
        
        // Declare a mutable variable named guess to store user input
        // Create a new empty String and bind it to the guess variable
        let mut guess = String::new();
        
        /*
        Receive user input from the standard input (keyboard) and store it in the guess variable
        The read_line method reads input from the standard input and appends it to the guess string
        The &mut guess argument indicates that read_line should store the input in the guess variable
        The expect method is called in case of an error during input reading
        It displays the provided error message if an error occurs
        */
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        /*
        Compare the user's guess with the secret number and print the result
        - guess.trim() removes any leading or trailing whitespace from the user input
        - parse() method parses the trimmed input into an unsigned 32-bit integer (u32)
        - If parsing is successful (Ok), the parsed number is assigned to the guess variable
        - If parsing fails (Err), the loop continues to the next iteration
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // If parsing is successful, assign the parsed number to guess
            Err(_) => continue, // If parsing fails, continue to the next iteration of the loop
        };
        
        /*
        Print the user's guess by using string interpolation
        The {} is a placeholder for the value of the guess variable
        */
        println!("You guessed: {}", guess);
    
        /*
        Match the result of comparing the user's guess with the secret number
        - cmp() method compares the guess with the secret number and returns an Ordering enum variant
        - The match expression is used to handle the different outcomes of the comparison
        */
        match guess.cmp(&secret_number) {
            // If the guess is less than the secret number
            Ordering::Less => println!("Too small! Try again."),
            // If the guess is greater than the secret number
            Ordering::Greater => println!("Too big! Try again."),
            // If the guess is equal to the secret number
            Ordering::Equal => {
                println!("It's a match! You win!");
                // Break out of the loop as the game is won
                break;
            }
        }
    }
}
