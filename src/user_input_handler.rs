use std::io;
use std::io::Write;
use crate::utils::{format_message, Args, commit, add_all};

pub fn handle_with_user_input() {
    // TODO: Add "popup" for SUGGESTED_TYPES
    let message_type = get_user_input("What type of commit is this?", true);
    let message = get_user_input("What is the commit message?", true);
    let scope = get_user_input("What is the scope of this commit? (Optional)", false);

    let body = get_user_input(
        "What is the body of this commit? (// = new paragraph) (Optional)",
        false,
    );

    let body: Vec<String> = body
        .split("//")
        .map(|s| format!("-m {}", s.trim()))
        .collect();

    let all = get_user_input("Do you want to commit all files? (y/N)", false);
    let all = match all.to_lowercase().as_str() {
        "y" | "yes" => {
            add_all();
            true
        }
        _ => false,
    };

    let footer = get_user_input("What is the footer of this commit? (Optional)", false);

    let breaking = get_user_input("Is this a breaking change? (y/N)", false);
    let breaking = match breaking.to_lowercase().as_str() {
        "y" | "yes" => {
            let change = get_user_input("What is the breaking change?", true);
            format!("-m 'BREAKING CHANGE: {}'", change)
        }
        _ => "".to_owned(),
    };

    let message = format_message(message_type, scope, message, breaking.clone());

    let mut args = vec![message];
    args.extend(body);
    args.push(breaking);
    args.push(format!("-m {}", footer));
    args.retain(|x| !x.is_empty());

    commit(
        args,
        Args {
            message: String::new(),
            message_type: String::new(),
            scope: None,
            body: None,
            footer: None,
            breaking: None,
            amend: false,
            all,
        },
    );
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
