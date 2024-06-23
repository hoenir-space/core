use colored::Colorize;
use crate::VERSION;

pub mod logo;

pub fn hello(){

    println!(
        "[ {} CLI  | {:<17} | {:^20} | {:^20} ]",
        "Hœnir".blue().bold(),
        format!( "version {}", VERSION.blue()),
        "-",
        "-"
    );

}