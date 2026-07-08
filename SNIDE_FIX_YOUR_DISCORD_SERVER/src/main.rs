use std::{io, thread::sleep, time::Duration};

fn main() {
    for _ in 1..=100_u8 {
        println!("SNIDE FIX UR DC");
        sleep(Duration::from_millis(5));
    }

    let mut input = String::new();

    loop {
        input.clear();

        println!("\nSnide are you sexy? (y/n)");

        io::stdin()
            .read_line(&mut input)
            .expect("program fucked up");

        match input.trim().to_ascii_lowercase().as_str() {
            "n" => {
                println!("Korrect™");
                break;
            }
            "y" => println!("Inkorrect™"),
            "" => println!("Input something dawg™"),
            _ => println!("Not a listed option™"),
        }
    }
}
