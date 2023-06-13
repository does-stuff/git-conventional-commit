use clap::Parser;
use std::io;
use std::io::Write;

const SUGGESTED_TYPES: [&str; 10] = [
    "fix", "feat", "build", "chore", "ci", "docs", "style", "refactor", "perf", "test",
];

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

    if args.len() > 1 {
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

    let message = format_message(args.message_type, scope, args.message);
    println!("{}", message);
}

fn handle_with_user_input() {
    // TODO: Add "popup" for SUGGESTED_TYPES
    let message_type = get_user_input("What type of commit is this?", true);
    let message = get_user_input("What is the commit message?", true);
    let scope = get_user_input("What is the scope of this commit? (Optional)", false);

    let message = format_message(message_type, scope, message);
    println!("{}", message);
}

fn get_user_input(prompt: &str, required: bool) -> String {
    let mut input = String::new();
    print!("{prompt}: ");
    io::stdout().flush().unwrap(); // Print before read_line

    io::stdin().read_line(&mut input).unwrap();

    if required && input.trim().is_empty() {
        println!("Please type an input!");
        return get_user_input(prompt, required);
    }

    return input.trim().to_owned();
}

fn format_message(message_type: String, scope: String, message: String) -> String {
    let scope = match scope.trim().is_empty() {
        true => "".to_owned(),
        false => format!("({})", scope).to_string(),
    };

    return format!("{}{}: {}", message_type, scope, message);
}

fn commit(args: Vec<String>) {
    std::process::Command::new("git")
        .arg("commit")
        .arg(args.join(" "))
        .spawn()
        .expect("Failed to commit");
}
