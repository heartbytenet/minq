#[macro_use] extern crate rocket;

use std::collections::VecDeque;
use std::sync::Mutex;

static QUEUE: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

#[get("/")]
fn route_index() -> &'static str {
  "Hello, world!"
}

#[post("/")]
fn route_pull() -> String {
  let mut queue = QUEUE.lock().unwrap();
  let item = queue.pop_front().unwrap_or_else(|| "".to_string());
  item
}

#[post("/", data = "<input>")]
fn route_push(input: String) -> String {
  let mut queue = QUEUE.lock().unwrap();
  queue.push_back(input);
  "push".to_string()
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![route_index])
    .mount("/pull", routes![route_pull])
    .mount("/push", routes![route_push])
}
