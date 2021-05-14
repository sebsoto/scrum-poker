#![feature(proc_macro_hygiene, decl_macro)]

mod server;


#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::State;
use rocket::request::Form;
use std::error::Error;
use serde_json;

#[derive(FromForm)]
struct NewSessionForm{
    name: String,
}

#[derive(FromForm)]
struct Vote{
    name: String,
    value: usize,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/sessions")]
fn session_list(sp: State<server::ScrumPoker>) -> String {
    sp.list_sessions().unwrap().join(",")
}

#[get("/sessions/<session_name>")]
fn get_results(sp: State<server::ScrumPoker>, session_name: &RawStr) -> String {
    let results = sp.get_results(session_name).unwrap();
    serde_json::to_string(&results).unwrap()

}

#[put("/sessions/<session_name>", data = "<vote>")]
fn vote(sp: State<server::ScrumPoker>, session_name: &RawStr, vote: Form<Vote>) -> Result<(), Box<dyn Error>> {
    sp.vote(session_name, vote.name.clone(), vote.value)
}

#[post("/sessions", data = "<session_request>")]
fn new_session(sp: State<server::ScrumPoker>, session_request: Form<NewSessionForm>) -> Result<(), Box<dyn Error>> {
    sp.add_session(session_request.into_inner().name)
}

fn main() {
    let sp = server::ScrumPoker::new(10);
    rocket::ignite().manage(sp).mount("/", routes![index, session_list, get_results, new_session, vote]).launch();
}