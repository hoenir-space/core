use colored::Colorize;
use crate::VERSION;

pub mod logo;

pub fn hello(){

    println!(
        "[ {} CLI   | version {:<10} | {:^10} | {:^28} ]",
        "Hœnir".blue().bold(),
        VERSION.blue(),
        "-",
        "(c) G0los 2024 - MIT License"
    );

}