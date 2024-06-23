use std::ops::{Add, Sub};
use colored::Colorize;
use crate::VERSION;
use std::time::{Duration, SystemTime};
pub mod logo;



fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH.add(Duration::new(1708729200,0))) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn hello(){

    println!(
        "[ {} CLI | version {:<10} | {:^12} | {:^28} ]",
        "Hœnir".blue().bold(),
        VERSION.blue(),
        get_sys_time_in_secs().to_string(),
        "(c) G0los 2024 - MIT License"
    );

}