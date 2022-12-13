use clap::{ Arg, Command };

pub fn init() -> Command {
    Command::new("open")
        .subcommand(Command::new("pwd"))
        .subcommand(Command::new("ls"))
        .subcommand(
            Command::new("add")
                .arg(Arg::new("name").required(true))
                .arg(Arg::new("target").required(true))
        )
        .subcommand(Command::new("remove").arg(Arg::new("name").required(true)))
        .arg(Arg::new("open").required(true))
        .args_conflicts_with_subcommands(true)
}