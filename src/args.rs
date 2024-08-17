use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ReminderArgs {
    #[clap(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Open TUI
    Interactive(InteractiveCommand),
    /// Create new reminder in TUI or with arguments
    New(NewCommand),
    /// Import .ics file
    Import(ImportCommand),
    /// Print all reminders at for a date or date range (default today)
    List(ListCommand),
    /// Grep for a reminder
    Grep(GrepCommand),
    /// New command but with an existing reminder as a template
    Copy(CopyCommand),
    /// Move reminder(s) to another calendar
    Move(MoveCommand),
    /// Mark reminder(s) as cancelled
    Cancel(ActionCommand),
    /// Edit reminder in TUI
    Edit(EditCommand),
    /// Delete reminder(s). This is PERMANENT.
    Delete(ActionCommand),
    /// Mark reminder(s) as done
    Done(ActionCommand),
    /// Show all info about reminder(s)
    Info(ActionCommand),
}

#[derive(Debug, Args)]
pub struct ActionCommand {
    #[arg(num_args=1..)]
    reminders: Vec<i32>
}

#[derive(Debug, Args)]
pub struct EditCommand {
    #[arg(short, long)]
    reminder: i32
}

#[derive(Debug, Args)]
pub struct MoveCommand {
    #[arg(short, long)]
    calendar: String,

    #[arg(num_args=1..)]
    reminders: Vec<i32>
}

#[derive(Debug, Args)]
pub struct NewCommand {
    #[arg(short, long)]
    calendar: Option<String>,
    #[arg(short, long)]
    start: Option<String>,
    #[arg(short, long)]
    due: Option<String>,
    #[arg(short, long)]
    location: Option<String>,
    #[arg(short, long)]
    priority: Option<i32>,
    #[arg(short, long)]
    category: Option<String>,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    summary: Option<String>,
    #[arg(short, long, default_value=false)]
    tui: bool
}

#[derive(Debug, Args)]
pub struct ImportCommand {

}

#[derive(Debug, Args)]
pub struct CopyCommand {

}

#[derive(Debug, Args)]
pub struct GrepCommand {

}

#[derive(Debug, Args)]
pub struct ListCommand {

}

#[derive(Debug, Args)]
pub struct InteractiveCommand {

}

