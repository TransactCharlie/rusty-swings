use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn get_guess() -> u32 {
    loop {
        // Note if we don't shadow this every time we'll
        // end up with guess getting *APPENDED TO* by
        // io::stdin().read_line()
        let mut guess = String::new();

        println!("Please guess a number:");

        io::stdin().read_line(&mut guess)
            .expect("Failed to Read Line!");

        let _guess: u32 = match guess.trim().parse() {
            Ok(num) => return num,
            Err(_) => continue,
        };
    }
}


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess;

    loop {
        guess = get_guess();
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
