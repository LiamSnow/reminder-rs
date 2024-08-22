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
            let _ = read_calendars();
        }
        _ => {}
    }
}

fn read_calendars() -> std::io::Result<()> {
    let test_calendars_path = Path::new("test_calendars");

    if !test_calendars_path.is_dir() {
        println!("Error: 'test_calendars' directory not found");
        return Ok(());
    }

    for entry in fs::read_dir(test_calendars_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            let calendar_name = path.file_name().unwrap();
            let subfolder = path.join(calendar_name);
            let vtodos = (if subfolder.is_dir() {
                read_vtodos(&subfolder)
            } else {
                read_vtodos(&path)
            })
            .unwrap();

            println!("Calendar: {}", calendar_name.to_string_lossy());

            for vtodo in vtodos {
                println!(" - todo: {}", vtodo.summary.unwrap());
            }
        }
    }

    Ok(())
}

fn read_vtodos(dir_path: &PathBuf) -> io::Result<Vec<VTodo>> {
    let mut vtodos = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("ics") {
            if let Ok(file) = fs::File::open(&path) {
                let reader = io::BufReader::new(file);
                let mut lines = reader.lines();

                while let Some(Ok(line)) = lines.next() {
                    if line.trim() == "BEGIN:VTODO" {
                        if let Some(vtodo) = parse_vtodo(&mut lines) {
                            vtodos.push(vtodo);
                        }
                    }
                }
            }
        }
    }

    Ok(vtodos)
}

fn parse_vtodo<B: BufRead>(lines: &mut std::io::Lines<B>) -> Option<VTodo> {
    let mut vtodo = VTodo {
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
            return Some(vtodo);
        }

        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
            match parts[0] {
                "COMPLETED" => vtodo.completed = Some(parts[1].to_string()),
                "CREATED" => vtodo.created = Some(parts[1].to_string()),
                "DTSTAMP" => vtodo.dtstamp = Some(parts[1].to_string()),
                "LAST-MODIFIED" => vtodo.last_modified = Some(parts[1].to_string()),
                "PERCENT-COMPLETE" => vtodo.percent_complete = parts[1].parse().ok(),
                "PRIORITY" => vtodo.priority = parts[1].parse().ok(),
                "SEQUENCE" => vtodo.sequence = parts[1].parse().ok(),
                "STATUS" => vtodo.status = Some(parts[1].to_string()),
                "DESCRIPTION" => vtodo.description = Some(parts[1].to_string()),
                "SUMMARY" => vtodo.summary = Some(parts[1].to_string()),
                "UID" => vtodo.uid = Some(parts[1].to_string()),
                _ => {}
            }
        }
    }

    None
}
