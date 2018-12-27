#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[post("/node", data = "<input>")]
fn node(input: String) -> String {
    let _x = input; // we dont do anything with the input, this sqwelches the warning
    "{ status: \"ok\" }".to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![node]).launch();
}