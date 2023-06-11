use clap::{arg, Command, ArgAction};

fn main() {
    let matches = Command::new("oi")
        .about("Get AI powered short answers to questions")
        .version("0.1.0")
        .arg(
            arg!(--configure "Opens the configuration file in your $EDITOR")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            arg!([question] "The question to ask the AI")
                .required(false),
        )
        .get_matches();

    if matches.get_flag("configure") {
        println!("Opening configuration...");
    } else if let Some(question) = matches.get_one::<String>("question") {
        println!("Question: {}", question);
    }
}

