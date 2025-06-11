use std::io; // imports the io (input/output) library from std which means standard
             // Basically std = standard

fn main() { // start of the main function
    println!("Welcome to the guessing game!"); // Basic print statement
    println!("Please put your guess!");

    let mut guess = String::new(); // Makes a variable that is mutable (it can be changed)
                                   // The string part at the end if defininf a string value
                                   // which is basically just saying this variable is a string.

    io::stdin() // Accessing the input/output library

        .read_line(&mut guess) // i'm assuming this is calling the guess variable we made earlier
                               // and making sure that it's mutable.

        .expect("Failed to read line"); // This looks like an error handling line, if something
                                        // dosen't go as expected then catch the error basically
                                        // and then print Failed to read line.

    println!("You guessed: {}", guess); // Print the result
}

