#![feature(proc_macro_hygiene, decl_macro)]
use structopt::StructOpt;

#[macro_use] extern crate rocket;

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
    #[structopt(short = "c", long = "count", default_value = "100")]
    payload_count: u32
}

#[post("/node", data = "<input>")]
fn node(input: String) -> String {
    let opt = Opt::from_args();
    let client = reqwest::Client::new();
    let mut vec = vec![0u8; input.len()];
    vec.clone_from_slice(&input.as_bytes());
    let _ = client.post(opt.output.as_str())
        .body(vec)
        .send().unwrap();
    "{ status: \"ok\" }".to_string()
}

fn main() {
    rocket::ignite().mount("/", routes![node]).launch();
}