#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::config::{Config, Environment};

#[post("/node", data = "<input>")]
fn node(input: String) -> String {
    let _x = input; // we dont do anything with the input, this sqwelches the warning
    "{ status: \"ok\" }".to_string()
}

fn main() {
    let config = Config::build(Environment::Staging)
    .address("192.168.69.100")
    .port(8000)
    .finalize().unwrap();
    let app = rocket::custom(config);
    app.mount("/", routes![node]).launch();
    // rocket::ignite().mount("/", routes![node]).launch();
    // app::ignite().mount("/", routes![node]).launch();
}