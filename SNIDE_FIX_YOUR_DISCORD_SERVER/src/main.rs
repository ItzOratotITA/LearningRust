use {
    anstream::{print, println}, // Microsoft are we genuinely being for real right now?
    std::io::{self, Write},
};
mod ansictrlcodes;
mod ask_shit_yn;

fn main() {
    print!(
        "{}\n{}  /$$$$$$  /$$   /$$ /$$$$$$ /$$$$$$$  /$$$$$$$$
 /$$__  $$| $$$ | $$|_  $$_/| $$__  $$| $$_____/
| $$  \\__/| $$$$| $$  | $$  | $$  \\ $$| $$      
|  $$$$$$ | $$ $$ $$  | $$  | $$  | $$| $$$$$   
 \\____  $$| $$  $$$$  | $$  | $$  | $$| $$__/   
 /$$  \\ $$| $$\\  $$$  | $$  | $$  | $$| $$      
|  $$$$$$/| $$ \\  $$ /$$$$$$| $$$$$$$/| $$$$$$$$
 \\______/ |__/  \\__/|______/|_______/ |________/{}{}
  _____ _                     ____   ____ 
 |  ___(_)_  __  _   _ _ __  |  _ \\ / ___|
 | |_  | \\ \\/ / | | | | '__| | | | | |    
 |  _| | |>  <  | |_| | |    | |_| | |___ 
 |_|   |_/_/\\_\\  \\__,_|_|    |____/ \\____|{}{}


 This is a JOKE, don't take it seriously! :D{}
 {}Press ENTER to play!{} {}(and continue when you fail){} ",
        ansictrlcodes::CLEARSCREEN,
        ansictrlcodes::GREEN,
        ansictrlcodes::RESET,
        ansictrlcodes::BLURPLE,
        ansictrlcodes::RESET,
        ansictrlcodes::BLUE,
        ansictrlcodes::RESET,
        ansictrlcodes::ORANGE,
        ansictrlcodes::RESET,
        ansictrlcodes::RED,
        ansictrlcodes::RESET,
    );
    io::stdout().flush().expect("could not flush stdout");
    rpassword::read_password().expect("program fucked up, rpassword crate was so not needed lmfao"); // With all those damn crates file sizes gonna be 10 petabytes
    let mut total_fails: u16 = 0;
    println!();
    ask_shit_yn::ask_shit_yn(
        "Snide will you enable viewing message history on your Discord server?",
        true,
        1,
        3,
        &mut total_fails,
    );
    ask_shit_yn::ask_shit_yn(
        "Snide will you fix your broken link to YouTube on your guns.lol?",
        true,
        2,
        3,
        &mut total_fails,
    );
    ask_shit_yn::ask_shit_yn("Snide are you sexy?", false, 3, 3, &mut total_fails);
    println!(
        "\n BTW the thing wrong with your guns.lol is that you have to type '.../@Snide21' not '.../Snide21'"
    )
}
