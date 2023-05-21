// Items defined in the standard library which is in the scope
// of every program.
// This set is called the prelude
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // macro's finish with a !
    println!("Guess the number!");


    loop {
        println!("Please input your guess");
        let mut guess = String::new();  // UTF-8 encoded and growable.
                                        // ::new() is an "associated function" of the String type

        // ie. also invokable with std::io::stdin()
        io::stdin()
            .read_line(&mut guess) // appends user input to the string
            .expect("Failed to read line"); // expect is a method of Result !
                                            // The exercice adds this line to avoid the compiler warning.
                                            // In a real program, you would handle the error.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // refers to the loops 'continue'. ie. go to the next iteration
        };

    // A match expression is made up of arms.
    // An arm consists of a pattern to match against, and the code that should be run if the
    // value given to match fits that arm’s pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Congratulations! You guessed the secret number was {secret_number}");
                break;
            },
        }
    }
}


// QUESTION:
// - What is the difference between variable binding and assignment?
//   ANSWER: Variable binding is the process of associating a name with a value.
//           Assignment is the process of giving a variable a value.
// - What is the difference between a method and an associated function?
//   ANSWER: Associated functions are functions associated with a type.
//           Methods are functions associated with an instance of a type.

// LEARN:
// 1. enum stands for enumeration !
// 2. each state of an enum is called a variant
// eg. Result is an enum with variants Ok and Err.
// 3. There is a difference between a library crate and a binary crate.
//    A library crate cannot be executed on its own.
//    A binary crate is a crate that can be compiled into an executable.
