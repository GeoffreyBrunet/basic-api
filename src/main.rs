#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::process::{Command, Stdio};

#[get("/<get_letter>/<nb_repeat>")]
fn index(get_letter: String, nb_repeat: usize) -> String {
    std::iter::repeat(get_letter).take(nb_repeat -1).collect::<String>()
}

#[get("/name")]
fn name() -> String {
    let output = Command::new("hostname")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
}

fn main() {
    rocket::ignite().mount("/", routes![index, name]).launch();
}
