use colored::Colorize;
use crate::VERSION;

pub mod logo;

pub fn hello(){

    println!(
        "[ {} CLI   | version {:<10} | {:^20} | {:^18} ]",
        "Hœnir".blue().bold(),
        VERSION.blue(),
        "-",
        "© G0los 2024"
    );

}