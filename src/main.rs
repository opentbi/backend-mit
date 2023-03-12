extern crate serde_json;

mod config;

use crate::config::Config;

fn main() {
    let cfg = Config::load();

    println!("{:?}", cfg);
}
