# Guessing Game

This is a simple guessing game implemented in Rust. The program generates a random number between 1 and 100, and the user has to guess the number. After each guess, the program provides feedback whether the guess is too small, too big, or correct.

## How to Play

1. Clone this repository to your local machine.
2. Make sure you have Rust installed. If not, you can download it from [here](https://www.rust-lang.org/tools/install).
3. Navigate to the directory containing the cloned repository.
4. Build the project using the `cargo build` command.
5. Run the program using the `cargo run` command.

## Instructions

- The program will prompt you to input your guess.
- Enter your guess and press Enter.
- If your guess is too small, the program will inform you.
- If your guess is too big, the program will inform you.
- If your guess is correct, the program will display a victory message and terminate.

## Dependencies

- This project relies on the `rand` crate for generating random numbers.
- Make sure to include the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8.4"
