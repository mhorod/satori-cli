use clap::{Arg, ArgAction, Command};

pub fn build_cli() -> Command {
    Command::new("satori")
        .about("Satori CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(username_command())
        .subcommand(contests_command())
        .subcommand(details_command())
        .subcommand(logout_command())
        .subcommand(problems_command())
        .subcommand(pdf_command())
        .subcommand(results_command())
        .subcommand(status_command())
        .subcommand(submit_command())
}

fn username_command() -> Command {
    Command::new("username").about("Show username")
}

fn contests_command() -> Command {
    Command::new("contests")
        .about("List contests")
        .arg(
            Arg::new("archived")
                .short('a')
                .long("archived")
                .action(ArgAction::SetTrue)
                .help("Show archived contests"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force refresh"),
        )
}

fn details_command() -> Command {
    Command::new("details")
        .about("Show details of submission")
        .arg(
            Arg::new("contest")
                .short('c')
                .long("contest")
                .action(ArgAction::Set)
                .required(true)
                .help("Prefix of contest name"),
        )
        .arg(
            Arg::new("submission")
                .short('s')
                .long("submission")
                .action(ArgAction::Set)
                .required(true)
                .help("Submission ID"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force refresh"),
        )
}

fn problems_command() -> Command {
    Command::new("problems")
        .about("List problems")
        .arg(
            Arg::new("contest")
                .short('c')
                .long("contest")
                .action(ArgAction::Set)
                .required(true)
                .help("Prefix of contest name"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force refresh"),
        )
}

fn pdf_command() -> Command {
    Command::new("pdf")
        .about("Download pdf")
        .arg(
            Arg::new("contest")
                .short('c')
                .long("contest")
                .action(ArgAction::Set)
                .required(true)
                .help("Prefix of contest name"),
        )
        .arg(
            Arg::new("problem")
                .short('p')
                .long("problem")
                .action(ArgAction::Set)
                .required(true)
                .help("Problem code"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force refresh"),
        )
}

fn submit_command() -> Command {
    Command::new("submit")
        .about("Submit solution")
        .arg(
            Arg::new("contest")
                .short('c')
                .long("contest")
                .action(ArgAction::Set)
                .required(true)
                .help("Prefix of contest name"),
        )
        .arg(
            Arg::new("problem")
                .short('p')
                .long("problem")
                .action(ArgAction::Set)
                .required(true)
                .help("Problem code"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .action(ArgAction::Set)
                .required(true)
                .help("Path to solution file"),
        )
}

fn status_command() -> Command {
    Command::new("status")
        .about("Show status of the problem")
        .arg(
            Arg::new("contest")
                .short('c')
                .long("contest")
                .action(ArgAction::Set)
                .required(true)
                .help("Prefix of contest name"),
        )
        .arg(
            Arg::new("problem")
                .short('p')
                .long("problem")
                .action(ArgAction::Set)
                .required(true)
                .help("Problem code"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force refresh"),
        )
        .arg(
            Arg::new("best")
                .short('b')
                .long("best")
                .action(ArgAction::SetTrue)
                .conflicts_with("recent")
                .help("Show best result"),
        )
        .arg(
            Arg::new("recent")
                .short('r')
                .long("recent")
                .action(ArgAction::SetTrue)
                .conflicts_with("best")
                .help("Show recent result"),
        )
}

fn results_command() -> Command {
    Command::new("results")
        .about("Show results of submitted solutions")
        .arg(
            Arg::new("contest")
                .short('c')
                .long("contest")
                .action(ArgAction::Set)
                .required(true)
                .help("Prefix of contest name"),
        )
        .arg(
            Arg::new("problem")
                .short('p')
                .long("problem")
                .action(ArgAction::Set)
                .help("Problem code"),
        )
        .arg(
            Arg::new("limit")
                .short('l')
                .long("limit")
                .action(ArgAction::Set)
                .default_missing_value("")
                .help("Limit number of results"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force refresh"),
        )
}

fn logout_command() -> Command {
    Command::new("logout").about("Logout from Satori")
}
