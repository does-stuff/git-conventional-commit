use crate::utils::{add_all, commit, format_message, Args};
use clap::Parser;

pub fn handle_with_clap() {
    let cli_args = Args::parse();

    if cli_args.all {
        add_all();
    }

    let scope = match cli_args.scope.clone() {
        Some(scope) => scope,
        None => "".to_owned(),
    };

    let breaking = match cli_args.breaking.clone() {
        Some(breaking) => breaking,
        None => "".to_owned(),
    };

    let message = format_message(
        cli_args.message_type.clone(),
        scope,
        cli_args.message.clone(),
        breaking,
    );

    let body = match cli_args.body.clone() {
        Some(body) => body,
        None => vec![],
    };

    let body: Vec<String> = body.iter().map(|s| format!("-m {}", s.trim())).collect();
    let body = body.join(" ");

    let footer = match cli_args.footer.clone() {
        Some(footer) => format!("-m {}", footer),
        None => "".to_owned(),
    };

    let breaking = match cli_args.breaking.clone() {
        Some(breaking) => format!("-m BREAKING CHANGE: {}", breaking),
        None => "".to_owned(),
    };

    let mut args = vec![message, body, breaking, footer];
    args.retain(|x| !x.is_empty());

    commit(args, cli_args);
}
