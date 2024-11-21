mod caldav;
mod ical;
mod args;

use args::*;
use caldav::client::{CalDAVClient, Calendar};
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
        ReminderSubcommands::List(ListCommand { calendar: calendar_name_opt }) => {
            match calendar_name_opt {
                Some(calendar_name) => {
                    let cal = client.get_calendar(calendar_name).unwrap();
                    print_todos(&client, cal).await;
                },
                None => {
                    for cal in &client.calendars {
                        print_todos(&client, &cal).await;
                    }
                },
            }
        }
        _ => {}
    }
}

async fn print_todos(client: &CalDAVClient, cal: &Calendar) {
    println!("Todos for {}", cal.fancy_name());

    let todos = client.get_current_todos(cal).await;
    for todo in todos {
        let summary = todo.get_summary().unwrap_or("Error");
        println!("{summary}");
    }
}
