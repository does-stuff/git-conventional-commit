use clap::Parser;
use std::io;
use std::io::Write;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'm', long = "message")]
    message: String,

    #[arg(short = 't', long = "type")]
    message_type: String,

    #[arg(short = 's', long = "scope")]
    scope: Option<String>,
}

fn main() {
    let args = std::env::args();

    if args.len() >= 1 {
        handle_with_clap();
    } else {
        handle_with_user_input();
    }
}

fn handle_with_clap() {
    let args = Args::parse();

    let scope = match args.scope {
        Some(scope) => format!("({})", scope).to_string(),
        None => "".to_owned(),
    };

    let message = format!("{}{}: {}", args.message_type, scope, args.message);
    println!("{}", message);
}

fn handle_with_user_input() {}

fn get_user_input(prompt: &str, required: bool) -> String {
    let mut input = String::new();
    print!("{prompt}: ");
    io::stdout().flush().unwrap(); // Print before read_line

    io::stdin().read_line(&mut input).unwrap();

    if required && input.trim().is_empty() {
        println!("Please type an input!");
        return get_user_input(prompt, required);
    }

    return input;
}
