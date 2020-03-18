use rand::Rng; // brings Rng trait into scope (has required methods for random number generators)
use std::cmp::Ordering; // brings Ordering type (enum) into the scope
use std::io; // brings io module into the scope

//main fn
fn main() {
    // creates a random numbers from 1-100 using current thread and is seeded from the OS
    let secret_number = rand::thread_rng().gen_range(1, 101); // gen_range method takes two numbers as args and generates a random number between them
    let mut tries = 0;
    println!("Guess the Number!"); // this is a comment
    println!("Take a guess."); // macro that prints to console22

    // infinite loop
    loop {
        tries += 1;
        let mut guess = String::new(); // binds a mutable variable to a string instance (copy)

        // stdin fn returns an instance (copy) of io::Stdin which is a type that represents a handle to std terminal input
        io::stdin() // call the stdin fn from io module
            .read_line(&mut guess) // calls read_line method on std input handle to get user input and passes a mutable reference to a string called guess (references allow functions to directly modify arg data)
            .expect("Failed to read line"); // calls expect method on read_line return value (io:Result)

        // shadows previous value of guess
        let guess: u32 = match guess.trim().parse() {
            // throw return value (Result) from parse into a match expression
            Ok(num) => num, // if the Result is Ok then return number in result to be stored as u32 guess
            Err(_) => {
                // _ is a catchall value (catches all parse method errors)
                println!("That's not even a number idiot."); // if result is an err than parse failed and it's not a number
                continue; // continue to finish this loop cycle
            }
        };
        // trim method removes any whitespace from the string
        // parse method parses string into a number
        // since parse can be used on multiple types we need to annotate the variable type being stored using ": type" on new variable

        println!("You guessed {} in {} tries.", guess, tries); // {} are string placeholders

        // match expression on the return value (Ordering enum) from cmp method
        match guess.cmp(&secret_number) {
            // cmp method takes a reference to the comparable value and returns a variant of Ordering enum (Ordering::Greater, Ordering::Less, or Ordering::Equal)
            Ordering::Less => println!("Too small!"), // pattern => what to do if pattern is met = match arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // breaks infinite loop
            }
        }

        println!("Take another guess!");
    }
}
