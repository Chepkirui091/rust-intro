use std::io;

fn main() { //entry into the project
    println!("Guess the number!"); //println! is a macro that prints a string to the screen:

    println!("Please input your guess.");

    let mut guess = String::new(); //we’ll create a mutable variable to store the user input, like this:  the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!

    io::stdin() //Constructs a new handle to the standard input of the current process.
        .read_line(&mut guess) //Locks this handle and reads a line of input, appending it to the specified buffer.
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}