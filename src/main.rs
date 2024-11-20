mod caldav;
mod ical;

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
        for todo in todos {
            if let Some(s) = todo.summary {
                println!("{}", s);
            }
        }

        break;
    }
}
