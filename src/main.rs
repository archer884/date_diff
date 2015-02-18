extern crate chrono;

use chrono::{UTC, Local, DateTime};

fn main() {
    let date = Local.ymd(2015, 2, 20);

    println!("{}", date - Local::now());
}
