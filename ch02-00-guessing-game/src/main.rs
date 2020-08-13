use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut guess = String::new();
        println!("Please input your guess.");

        // stdin returns type result (which we can use for error checking)
        // and we pass in mutable pointer to our guess, yipee!
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
            // Error catching
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess);
        
        // Comparison and matching
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
