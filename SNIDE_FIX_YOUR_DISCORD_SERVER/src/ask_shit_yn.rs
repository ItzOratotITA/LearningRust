use {
    crate::ansictrlcodes,
    anstream::print,
    std::io::{self, Write},
};

pub fn ask_shit_yn(question: &str, yisc: bool, qnum: u8, qtot: u8, total_fails: &mut u16) {
    let mut input = String::new();
    loop {
        print!(
            "{} {}Fails: {}{} |{} {}/{} {}| {} {}(y/n){}: ",
            ansictrlcodes::CLEAR_CURRENT,
            ansictrlcodes::RED,
            *total_fails,
            ansictrlcodes::RESET,
            ansictrlcodes::ORANGE,
            qnum,
            qtot,
            ansictrlcodes::RESET,
            question,
            ansictrlcodes::BLUE,
            ansictrlcodes::RESET,
        );
        input.clear();
        io::stdout().flush().expect("could not flush stdout");

        let kontrol_d = io::stdin()
            .read_line(&mut input)
            .expect("program fucked up");

        if kontrol_d == 0 {
            panic!("you weren't supposed to do that");
        }

        let input = input.trim().to_ascii_lowercase();

        let (message, color, col2, correct, listed) = match input.as_str() {
            "y" | "yes" if yisc => (
                "Korrect™",
                ansictrlcodes::GREEN,
                ansictrlcodes::BOLD,
                true,
                true,
            ),
            "n" | "no" if !yisc => (
                "Korrect™",
                ansictrlcodes::GREEN,
                ansictrlcodes::BOLD,
                true,
                true,
            ),

            "y" | "yes" | "n" | "no" => (
                "Inkorrect™",
                ansictrlcodes::RED,
                ansictrlcodes::BOLD,
                false,
                true,
            ),

            "" => (
                "Input something dawg™",
                ansictrlcodes::ORANGE,
                "",
                false,
                false,
            ),

            _ => (
                "Not a listed option™",
                ansictrlcodes::ORANGE,
                "",
                false,
                false,
            ),
        };

        if !correct && listed {
            *total_fails += 1;
        }
        print!(
            "{} {}Fails: {}{} |{} {}/{} {}| {} {}(y/n){}: {}{}{}{}",
            ansictrlcodes::REWRITE_PREVIOUS,
            ansictrlcodes::RED,
            *total_fails,
            ansictrlcodes::RESET,
            ansictrlcodes::ORANGE,
            qnum,
            qtot,
            ansictrlcodes::RESET,
            question,
            ansictrlcodes::BLUE,
            ansictrlcodes::RESET,
            color,
            col2,
            message,
            ansictrlcodes::RESET,
        );
        io::stdout().flush().expect("could not flush stdout");

        if correct {
            println!();
            break;
        }

        rpassword::read_password()
            .expect("program fucked up, rpassword crate was so not needed lmfao");
        print!("{}", ansictrlcodes::REWRITE_PREVIOUS);
        io::stdout().flush().expect("could not flush stdout");
    }
}
