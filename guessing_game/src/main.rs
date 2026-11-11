// import io library
// std -> standard library
use std::io;
use std::cmp::Ordering;

use rand::prelude::*;


fn main() {

	println!("Guess a number!");

	let secret_number = rand::rng().random_range(1..=100);

	//println!("secret num: {secret_number}");

	//println!("Please input your guess");

	loop {
		println!("Please input your guess");

		// let used to create variables
		// immutable by default but adding mut
		// makes them mutable

		// let apples = 5; 		variable stuck at 5
		// let mut bananas = 5;		variable can now change
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {guess}");
		
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
