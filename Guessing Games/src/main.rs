extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("\nGuess the number!\n");

    loop {
        let secret_number = rand::thread_rng().gen_range(0, 21);

        println!("The secret number is: {}", secret_number);

        //   let mut chance = 0;

        for _i in 0..3 {
            // chance += 1;

            //     if chance > 3 {
            //         println!("You Loose !");
            //         break;
            //     }

            println!("Please input your guess.\n");

            let mut guess = String::new();

            // User Input
            stdin().read_line(&mut guess).expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    panic!("Please Enter a numbers only");
                }
            };
            println!("{} Attemps Left", _i);

            // println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small\n"),
                Ordering::Greater => println!("Too big\n"),
                Ordering::Equal => {
                    println!("You win\n");
                    break;
                } // Break
            } // match
        } // inner loop

        println!("You Loose !\n");

        println!("Do you want to play again (y/n).\n");

        let mut again = String::new();

        // User Input
        stdin().read_line(&mut again).expect("Failed to read line");

        if again == "y\n".to_string() {
            println!("\nGood luck\n");
        } else {
            println!("Thank You. !\n");
            break;
        }
    } // main Loop
} // Final Code Guessing Game.)
