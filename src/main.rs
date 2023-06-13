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

    #[arg(short = 't', long = "type", value_parser = SUGGESTED_TYPES)]
    message_type: String,

    #[arg(short = 's', long = "scope")]
    scope: Option<String>,

    #[arg(short = 'b', long = "body")]
    body: Option<Vec<String>>,

    #[arg(short = 'f', long = "footer")]
    footer: Option<String>,

    #[arg(short = None, long = "breaking")]
    breaking: Option<String>,

    #[arg(short = 'a', long = "amend")]
    amend: Option<bool>,
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
    let cli_args = Args::parse();

    let scope = match cli_args.scope.clone() {
        Some(scope) => scope,
        None => "".to_owned(),
    };

    let message = format_message(
        cli_args.message_type.clone(),
        scope,
        cli_args.message.clone(),
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

fn handle_with_user_input() {
    // TODO: Add "popup" for SUGGESTED_TYPES
    let message_type = get_user_input("What type of commit is this?", true);
    let message = get_user_input("What is the commit message?", true);
    let scope = get_user_input("What is the scope of this commit? (Optional)", false);

    let message = format_message(message_type, scope, message);

    let body = get_user_input(
        "What is the body of this commit? (// = new paragraph) (Optional)",
        false,
    );

    let body: Vec<String> = body
        .split("//")
        .map(|s| format!("-m {}", s.trim()))
        .collect();

    let footer = get_user_input("What is the footer of this commit? (Optional)", false);

    let breaking = get_user_input("Is this a breaking change? (y/N)", false);
    let breaking = match breaking.to_lowercase().as_str() {
        "y" => {
            let change = get_user_input("What is the breaking change?", true);
            format!("-m 'BREAKING CHANGE: {}'", change)
        }
        _ => "".to_owned(),
    };

    let mut args = vec![message];
    args.extend(body);
    args.push(breaking);
    args.push(format!("-m {}", footer));
    args.retain(|x| !x.is_empty());

    commit(
        args,
        Args {
            message: "".to_owned(),
            message_type: "".to_owned(),
            scope: None,
            body: None,
            footer: None,
            breaking: None,
            amend: None,
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

fn format_message(message_type: String, scope: String, message: String) -> String {
    let scope = match scope.trim().is_empty() {
        true => "".to_owned(),
        false => format!("({})", scope).to_string(),
    };

    return format!(
        "-m {}{}: {}",
        message_type.replace("'", "\""),
        scope.replace("'", "\""),
        message.replace("'", "\"")
    );
}

fn commit(args: Vec<String>, original_args: Args) {
    let mut command = std::process::Command::new("git");
    command.arg("commit").args(args);

    if original_args.amend.is_some() {
        command.arg("--amend");
    }

    command.spawn().expect("Failed to commit");
}

// Example commit with all options
