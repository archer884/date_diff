#![feature(env)]

extern crate chrono;

use chrono::{ Date, Local, TimeZone, };

fn main() {
    let date = match parse_date(std::env::args().nth(1)) {
        Ok(date) => date,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("Days remaining: {}", (date - Local::today()).num_days());
}

fn parse_date(input: Option<String>) -> Result<Date<Local>, String> {
    match input {
        Some(input) => {
            let values = input.split('/').filter_map(|i| i.parse().ok()).collect::<Vec<i32>>();

            if values.len() != 3 {
                return Err("Date must be of form yyyy/MM/dd".to_string());
            }

            Ok(Local.ymd(values[0], values[1] as u32, values[2] as u32))
        },
        _ => { Err("usage: ./date_diff yyyy/MM/dd".to_string()) },
    }
}
