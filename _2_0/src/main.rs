use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number:");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1..100);

    loop {
    let mut guess :String = String::new();
    io::stdin().read_line(&mut guess).expect("Failed Due to Error");

    println!("You guessed: {}", guess);
    println!("The secret number is: {}", secret_number);

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        },
    }
}
}
