use crate::cli;
use crate::cli::logo;

pub fn hello(){
    logo::print();
    cli::hello();
}