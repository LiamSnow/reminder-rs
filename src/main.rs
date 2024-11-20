mod caldav;
mod ical;
mod args;

use caldav::client::CalDAVClient;

#[tokio::main]
async fn main() {
    let client = CalDAVClient::new(
        "",
        "",
        ""
    ).await.unwrap();

    for cal in &client.calendars {
        println!("-- {} --", cal.name);
        let todos = client.get_todos(&cal.url).await;
        // for todo in todos {
        //     if let Some(s) = todo.summary {
        //         let is_not_complete = todo.percent.as_ref().map_or(true, |p| p != "100");
        //         if is_not_complete {
        //             println!("{}", s);
        //         }
        //     }
        // }
        println!("{}", todos.len());
    }
}
