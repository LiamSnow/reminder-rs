use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ReminderArgs {
    #[clap(subcommand)]
    pub subcommand: ReminderSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum ReminderSubcommands {
    /// Open TUI
    Interactive(InteractiveCommand),

    /// List calendars
    Calendars(CalendarsCommand),

    /// Create new reminder in TUI or with arguments
    New(NewCommand),
    /// Edit reminder in TUI or with args
    Edit(EditCommand),
    /// Copy a reminder and edit in TUI or with args
    Copy(CopyCommand),
    /// Import .ics file
    Import(ImportCommand),

    /// Print all incomplete reminders
    List(ListCommand),
    /// Search for a reminder
    Search(SearchCommand),
    /// Move reminder(s) to another calendar
    Move(MoveCommand),

    /// Mark reminder(s) as cancelled
    Cancel(ActionCommand),
    /// Delete reminder(s). This is PERMANENT.
    Delete(ActionCommand),
    /// Mark reminder(s) as done
    Done(ActionCommand),

    /// Show all info about reminder(s)
    Info(ActionCommand),

    /// TODO Remove
    Test(TestCommand)
}

#[derive(Debug, Args)]
pub struct CalendarsCommand {

}

#[derive(Debug, Args)]
pub struct TestCommand {
    pub pong: Option<i32>
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
    #[arg(short, long)]
    tui: bool
}

#[derive(Debug, Args)]
pub struct ImportCommand {
    #[arg(short, long)]
    pub calendar: String,
}

#[derive(Debug, Args)]
pub struct CopyCommand {

}

#[derive(Debug, Args)]
pub struct SearchCommand {
    #[arg(short, long)]
    pub calendar: Option<String>,
    #[arg(required = true)]
    pub term: String,
}

#[derive(Debug, Args)]
pub struct ListCommand {
    #[arg(short, long)]
    pub calendar: Option<String>,
}

#[derive(Debug, Args)]
pub struct InteractiveCommand {

}

