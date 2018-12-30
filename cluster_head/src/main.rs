#![feature(proc_macro_hygiene, decl_macro)]
use structopt::StructOpt;

#[macro_use] extern crate rocket;

use rocket::State;
use std::sync::Mutex;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "node")]
struct Opt {
    /// Enable more output
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Post location
    #[structopt(short = "o", long = "output")]
    output: String,
    /// Number of payloads to aggregate in memory before sending - 0 doesnt not aggregate
    #[structopt(short = "a", long = "aggregate", default_value = "0")]
    payload_count: usize
}


#[post("/node", data = "<input>")]
fn node(input: String, opt: State<Opt>, payloads: State<Mutex<Vec<Vec<u8>>>>) -> String {
    let client = reqwest::Client::new();
    let mut vec = vec![0u8; input.len()];
    vec.clone_from_slice(&input.as_bytes());
    if opt.payload_count > 0 {
        let mut payloads = payloads.lock().unwrap();
        payloads.push(vec);
        if payloads.len() > opt.payload_count {
            for vec in payloads.clone().into_iter() {
                let _ = client.post(opt.output.as_str())
                    .body(vec)
                    .send().unwrap();
            }
            payloads.clear();
        }
    } else {
        let _ = client.post(opt.output.as_str())
            .body(vec)
            .send().unwrap();
    }
    "{ status: \"ok\" }".to_string()
}

fn main() {
    let opt = Opt::from_args();
    let payloads: Mutex<Vec<Vec<u8>>> = Mutex::new(vec![]);
    rocket::ignite()
        .manage(opt)
        .manage(payloads)
        .mount("/", routes![node]).launch();
}