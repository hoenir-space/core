use std::ops::{Add, Sub};
use colored::Colorize;
use crate::VERSION;
use std::time::{Duration, SystemTime};
use thousands::Separable;
pub mod logo;



fn get_sys_time_in_secs() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH.add(Duration::new(1718632413,0))) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn hello(){

    let version_str = format!("version {}",VERSION);
    let time_str = format!("t= {}",get_sys_time_in_secs().separate_with_spaces());
    println!(
        "[  {}  | {:^20} | {:^17} | {:^18} ]",
        "Hœnir CLI".bright_blue().bold(),
        version_str,
        time_str,
        "© hoenir.space 2024"
    );

}