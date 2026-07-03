use std::io;

fn ask(
    question: &str,
    answer: &str,
    is_loop: bool,
    ask_for_retry: bool,
    easter_egg: bool,
    eecondition: &str,
    eeoutput: &str,
    ee_is_correct: bool,
) {
    let mut input = String::new();
    if !is_loop {
        println!("{question}");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        if input.trim().eq_ignore_ascii_case(answer) {
            print!("Korrect™\n");
        } else if easter_egg && input.trim().eq_ignore_ascii_case(eecondition) {
            print!("{eeoutput}")
        } else {
            println!("Inkorrect™ :(\n");
        }
    } else {
        loop {
            input.clear();
            print!("{question}");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");
            if input.trim().eq_ignore_ascii_case(answer) {
                print!("\nKorrect™\n\n");
                break;
            } else if easter_egg && input.trim().eq_ignore_ascii_case(eecondition) {
                print!("{eeoutput}");
                if !ee_is_correct {
                    if ask_for_retry {
                        if !ask_yn("Do you want to retry?", "Y", "n") {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                print!("\nInkorrect™ :(\n\n");
                if ask_for_retry {
                    if !ask_yn("Do you want to retry?", "Y", "n") {
                        break;
                    }
                }
            }
        }
    }
}

fn ask_yn(question: &str, default: &str, b: &str) -> bool {
    let mut yn_input = String::new();
    print!("{question} ({}/{b})\n", default.to_uppercase());
    io::stdin()
        .read_line(&mut yn_input)
        .expect("Failed to read input.");

    if yn_input.trim().eq_ignore_ascii_case(b) {
        return false;
    } else {
        return true;
    }
}

fn main() {
    loop {
        ask(
            "Who created Linux?\n",
            "Linus Torvalds",
            true,
            false,
            false,
            "",
            "",
            false,
        );
        ask(
            "What's 9+10?\n",
            "19",
            true,
            false,
            true,
            "21",
            "\nYou STOOPID!\n\n",
            false,
        );
        if !ask_yn("Retry?", "Y", "n") {
            break;
        }
    }
}
