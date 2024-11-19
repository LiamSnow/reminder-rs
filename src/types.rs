#[derive(Debug)]
pub struct VTodo {
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

pub struct Calendar {
  pub url: String,
  pub name: String,
  pub ctag: String
}
