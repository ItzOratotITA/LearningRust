use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn ask_yn(question: &str, a: &str, b: &str) -> bool {
    let mut yn_input = String::new();
    print!("{question} ({a}/{b})\n");
    io::stdin()
        .read_line(&mut yn_input)
        .expect("Failed to read input.");

    if yn_input.trim().to_lowercase() == b.to_lowercase() {
        return false;
    } else {
        return true;
    }
}

fn main() {
    println!("\n(yes this is from the book)\n\nNumber Guessing Game!\n");
    let ask_for_retry: bool = if ask_yn(
        "Do you want to automatically retry when you fail?\n(if not, you will be asked if you want to retry)",
        "Y",
        "n",
    ) {
        false
    } else {
        true
    };
    // thx LotB: onStackOverflow = for {"letting me know about this"} thing {"that saves time"}
    // https://stackoverflow.com/questions/59917293/how-to-access-rust-variables-outside-the-scope
    // not my question nor my issue (it would have been for the pretty dumb method I was thinking about using to do this)
    // but his answer was helpful anyways lmao

    let secret_number: u8 = rand::thread_rng().gen_range(1..=255);

    loop {
        println!("Guess the number :D (1-255):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input: NaN or too big.\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("\nToo smol!\n\n");
                if ask_for_retry && !ask_yn("Do you want to retry?", "Y", "n") {
                    break;
                }
            }
            Ordering::Greater => {
                println!("\nToo BEEG!\n\n");
                if ask_for_retry && !ask_yn("Do you want to retry?", "Y", "n") {
                    break;
                }
            }
            Ordering::Equal => {
                println!("\nYou win!");
                break;
            }
        }
    }
}
