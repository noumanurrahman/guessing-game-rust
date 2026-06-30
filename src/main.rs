use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut tries: i32 = 0;
    let number: i32 = rand::random_range(1..100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_num: i32 = guess
            .trim()
            .parse::<i32>()
            .unwrap_or_else(|_num| 0);

        tries += 1;

        match guess_num.cmp(&number) {
            Ordering::Less => println!("Guess bigger!"),
            Ordering::Greater => println!("Guess smaller!"),
            Ordering::Equal => {
                println!("You WON in {} tries!", tries);
                break;
            }
        }
    }
}
