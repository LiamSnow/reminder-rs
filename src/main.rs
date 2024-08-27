mod args;

use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use args::{ReminderArgs, ReminderSubcommands, TestCommand};
use clap::Parser;

#[derive(Debug)]
struct VTodo {
    completed: Option<String>,
    created: Option<String>,
    dtstamp: Option<String>,
    last_modified: Option<String>,
    percent_complete: Option<i32>,
    priority: Option<i32>,
    sequence: Option<i32>,
    status: Option<String>,
    description: Option<String>,
    summary: Option<String>,
    uid: Option<String>,
}

fn main() {
    let args = ReminderArgs::parse();

    match &args.subcommand {
        ReminderSubcommands::Test(TestCommand { pong: _ }) => {
            let calendars = read_calendars().unwrap();

            for calendar in calendars {
                println!("{}", calendar.file_name().unwrap().to_string_lossy());
                let todos = read_todos(&calendar).unwrap();

                for todo in todos {
                    println!(" - todo: {}", todo.summary.unwrap());
                }
            }
        }
        _ => {}
    }
}

fn read_calendars() -> Option<Vec<PathBuf>> {
    let test_calendars_path = Path::new("test_calendars");

    if !test_calendars_path.is_dir() {
        println!("Error: 'test_calendars' directory not found");
        return None;
    }

    let mut calendars: Vec<PathBuf> = vec![];

    for folder in fs::read_dir(test_calendars_path).ok()? {
        let folder = folder.ok()?;
        let folder_path = folder.path();

        if folder_path.is_dir() {
            let calendar_name = folder_path.file_name().unwrap();
            let subfolder_path = folder_path.join(calendar_name);
            if subfolder_path.is_dir() {
                calendars.push(subfolder_path);
            } else {
                calendars.push(folder_path);
            }
        }
    }

    Some(calendars)
}

fn read_todos(calendar_path: &PathBuf) -> io::Result<Vec<VTodo>> {
    let mut todos = Vec::new();

    for entry in fs::read_dir(calendar_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ics") {
            if let Ok(file) = fs::File::open(&path) {
                let reader = io::BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(Ok(line)) = lines.next() {
                    if line.trim() == "BEGIN:VTODO" {
                        if let Some(todo) = parse_todo(&mut lines) {
                            todos.push(todo);
                        }
                    }
                }
            }
        }
    }

    Ok(todos)
}

fn parse_todo<B: BufRead>(lines: &mut std::io::Lines<B>) -> Option<VTodo> {
    let mut todo = VTodo {
        completed: None,
        created: None,
        dtstamp: None,
        last_modified: None,
        percent_complete: None,
        priority: None,
        sequence: None,
        status: None,
        description: None,
        summary: None,
        uid: None,
    };

    while let Some(Ok(line)) = lines.next() {
        let line = line.trim();
        if line == "END:VTODO" {
            return Some(todo);
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            match parts[0] {
                "COMPLETED" => todo.completed = Some(parts[1].to_string()),
                "CREATED" => todo.created = Some(parts[1].to_string()),
                "DTSTAMP" => todo.dtstamp = Some(parts[1].to_string()),
                "LAST-MODIFIED" => todo.last_modified = Some(parts[1].to_string()),
                "PERCENT-COMPLETE" => todo.percent_complete = parts[1].parse().ok(),
                "PRIORITY" => todo.priority = parts[1].parse().ok(),
                "SEQUENCE" => todo.sequence = parts[1].parse().ok(),
                "STATUS" => todo.status = Some(parts[1].to_string()),
                "DESCRIPTION" => todo.description = Some(parts[1].to_string()),
                "SUMMARY" => todo.summary = Some(parts[1].to_string()),
                "UID" => todo.uid = Some(parts[1].to_string()),
                _ => {}
            }
        }
    }

    None
}
