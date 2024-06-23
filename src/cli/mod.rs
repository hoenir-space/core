use colored::Colorize;
use crate::VERSION;

pub mod logo;

pub fn hello(){

    println!(
        "[ {} CLI  | version {:<9} | {:^20} | {:^20} ]",
        "Hœnir".blue().bold(),
        VERSION.blue(),
        "-",
        "-"
    );

}