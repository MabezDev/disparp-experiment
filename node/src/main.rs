#![deny(warnings)]

extern crate reqwest;
extern crate env_logger;


use rand::prelude::*;

// #[macro_use] extern crate structopt;

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
    interval: u64
}

fn main() -> Result<(), Box<std::error::Error>> {
    env_logger::init();
    let opt = Opt::from_args();
    println!("{:?}", opt);
    let mut rng = rand::thread_rng();

    if opt.repeat {
        loop {
            run_req(&opt)?;
            let interval = if opt.interval == 0 {
                rng.gen_range(0u64, 1000u64)
            } else {
                opt.interval
            };
            thread::sleep(time::Duration::from_millis(interval));
        }
    } else {
        run_req(&opt)?;
    }

    

    println!("\n\nDone.");
    Ok(())
}

fn run_req(opt: &Opt) -> Result<(), Box<std::error::Error>> {
    let mut res = reqwest::get(opt.output.as_str())?;
    println!("Status: {}", res.status());

    // copy the response body directly to stdout
    if opt.debug {
        println!("Headers:\n{:?}", res.headers());
        std::io::copy(&mut res, &mut std::io::stdout())?;
    }
    Ok(())
}