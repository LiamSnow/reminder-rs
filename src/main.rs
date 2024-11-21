mod caldav;
mod ical;
mod args;

use args::*;
use caldav::client::CalDAVClient;
use clap::Parser;

#[tokio::main]
async fn main() {
    let client = CalDAVClient::new(
        "",
        "",
        ""
    ).await.unwrap();

    let args = ReminderArgs::parse();

    match &args.subcommand {
        ReminderSubcommands::Calendars(_) => {
            for cal in &client.calendars {
                println!("{} {}", cal.name, cal.color.clone().unwrap_or("no color".to_string()));
            }
        }
        ReminderSubcommands::List(ListCommand { calendar: calendar_name }) => {
            let cal = client.get_calendar(calendar_name).unwrap();
            println!("{} {} {}", cal.name, cal.url, cal.color.clone().unwrap_or("no color".to_string()));
            let todos = client.get_todos(cal).await;
            for todo in todos {
                println!("{}", todo.get_summary().unwrap_or("error".to_string()));
            }
        }
        _ => {}
    }
}
