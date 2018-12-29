#![deny(warnings)]

extern crate reqwest;
extern crate env_logger;


use rand::prelude::*;

use rayon::prelude::*;

use structopt::StructOpt;
use std::{thread, time};

#[derive(StructOpt, Debug)]
#[structopt(name = "node")]
struct Opt {
    /// Enable more output
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    /// Post location
    #[structopt(short = "o", long = "output")]
    output: String,
    /// Repeat the transmition
    #[structopt(short = "r", long = "repeat")]
    repeat: bool,
    /// Interval between repeat transmitions in ms - 0 will choose a random repeat value between 0 and 1000ms
    #[structopt(short = "i", long = "interval", default_value = "1000")]
    interval: u64,
    /// Number of 'nodes' to simulate
    #[structopt(short = "c", long = "count", default_value = "100")]
    nodes: u32
}

fn main() -> Result<(), Box<std::error::Error>> {
    env_logger::init();
    let opt = Opt::from_args();
    println!("{:?}", opt);
    
    let _ = (0..opt.nodes).into_par_iter().map(|i| {
        let mut rng = rand::thread_rng();
        let mut client = reqwest::Client::new();
        println!("Starting node {}", i);
        if opt.repeat {
            loop {
                run_req(&mut client, &opt).unwrap();
                let interval = if opt.interval == 0 {
                    rng.gen_range(0u64, 1000u64)
                } else {
                    opt.interval
                };
                thread::sleep(time::Duration::from_millis(interval));
            }
        } else {
            run_req(&mut client, &opt).unwrap();
        }
    });
    

    

    println!("\n\nDone.");
    Ok(())
}

fn run_req(client: &mut reqwest::Client, opt: &Opt) -> Result<(), Box<std::error::Error>> {
    let res = client.post(opt.output.as_str())
    .body(format!("{{ data: {} }}", 32))
    .send()?;
    println!("Status: {}", res.status());

    // copy the response body directly to stdout
    if opt.debug {
        println!("Headers:\n{:?}", res.headers());
    }
    Ok(())
}