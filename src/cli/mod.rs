use colored::Colorize;
use crate::VERSION;

pub mod logo;

pub fn hello(){

    println!(
        "{} {} {} {} {}.",
        "              This is the",
        "Hœnir".blue().bold(),
        "CLI".bold(),
        "with core library version",
        VERSION.blue()
    );

}