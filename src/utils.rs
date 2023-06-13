use clap::Parser;

pub const SUGGESTED_TYPES: [&str; 10] = [
    "fix", "feat", "build", "chore", "ci", "docs", "style", "refactor", "perf", "test",
];

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short = 'm', long = "message", help = "Add a message")]
    pub message: String,

    #[arg(short = 't', long = "type", value_parser = SUGGESTED_TYPES, help = "Add a type")]
    pub message_type: String,

    #[arg(short = 's', long = "scope", help = "Add a scope")]
    pub scope: Option<String>,

    #[arg(short = 'b', long = "body", help = "Add a body (can be chained)")]
    pub body: Option<Vec<String>>,

    #[arg(short = 'f', long = "footer", help = "Add a footer")]
    pub footer: Option<String>,

    #[arg(short = None, long = "breaking", help = "Is this a breaking change?")]
    pub breaking: Option<String>,

    #[arg(short = 'a', long = "amend", help = "Amend the last commit")]
    pub amend: bool,

    #[arg(short = None, long = "all", help = "Commit all files")]
    pub all: bool,
}

pub fn format_message(
    message_type: String,
    scope: String,
    message: String,
    breaking: String,
) -> String {
    let scope = match scope.trim().is_empty() {
        true => "".to_owned(),
        false => format!("({})", scope).to_string(),
    };

    return format!(
        "-m {}{}{}: {}",
        message_type.replace("'", "\""),
        scope.replace("'", "\""),
        if breaking.len() > 0 { "!" } else { "" },
        message.replace("'", "\"")
    );
}

pub fn add_all() {
    std::process::Command::new("git")
        .arg("add")
        .arg(".")
        .spawn()
        .expect("Failed to add all files");
}

pub fn commit(args: Vec<String>, original_args: Args) {
    let mut command = std::process::Command::new("git");
    command.arg("commit").args(args);

    if original_args.amend {
        command.arg("--amend");
    }

    command.spawn().expect("Failed to commit");
}
