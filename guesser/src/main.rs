use std::io; // input/output from stdard library
// python equivalent: from std import io
use rand::Rng; // from rand import RandomNumberGenerator dependencies
use std::cmp::Ordering; // import ordering funcs

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..=100); // create answer between 1 and 100

    loop{
        println!("Input guess: ");

        let mut guess = String::new(); //let creates a var, mut makes it mutable
        // :: indicates associated function of String type

        io::stdin() // call the in function of standard library. py eqiv: io.input()
            .read_line(&mut guess) // calls readline method on the input handle and passes
            // in guess to be re-written
            .expect("Failed to read guess"); // in-built exception handling; if not called will
            // produce warning

        // turn guess to number:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        // println!("You guessed: {}", guess); // output with {} placeholder
        // println!("The number was: {}", secret);

        // match the answers and get info about them, giving response
        match guess.cmp(&secret){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;},
        }
    }
}
