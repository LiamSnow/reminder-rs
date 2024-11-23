use std::cell::RefCell;

use args::*;
use caldav::client::{CalDAVClient, Calendar};
use clap::Parser;

mod caldav;
mod ical;
mod args;
mod tui;

#[tokio::main]
async fn main() {
    let client = CalDAVClient::new(
        "",
        "",
        ""
    ).await.unwrap();

    let args = ReminderArgs::parse();

    match &args.subcommand {
        ReminderSubcommands::Interactive(..) => {
            tui::start(client);
        }
        ReminderSubcommands::Calendars(_) => {
            for cal_ref in &client.calendars {
                let cal = cal_ref.borrow();
                println!("{}", cal.fancy_name());
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
                        print_todos(&client, cal).await;
                    }
                },
            }
        }
        ReminderSubcommands::Search(SearchCommand { calendar: calendar_name_opt, term }) => {
            match calendar_name_opt {
                Some(calendar_name) => {
                    let cal = client.get_calendar(calendar_name).unwrap();
                    search_todos(&client, cal, term).await;
                },
                None => {
                    for cal in &client.calendars {
                        search_todos(&client, cal, term).await;
                    }
                },
            }
        }
        _ => {}
    }
}

async fn search_todos(client: &CalDAVClient, cal_ref: &RefCell<Calendar>, term: &str) {
    let mut has_printed = false;
    let todos = client.get_current_todos(cal_ref).await;
    for todo in todos.as_ref() {
        let summary = todo.get_summary().unwrap_or("");
        if summary.contains(term) {
            if !has_printed {
                println!("Todos for {}", cal_ref.borrow().fancy_name());
                has_printed = true;
            }
            println!("{summary}");
        }
    }
}

async fn print_todos(client: &CalDAVClient, cal_ref: &RefCell<Calendar>) {
    println!("Todos for {}", cal_ref.borrow().fancy_name());

    let todos = client.get_current_todos(&cal_ref).await;
    for todo in todos.as_ref() {
        let summary = todo.get_summary().unwrap_or("Error");
        println!("{summary}");
    }
}



