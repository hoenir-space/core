use crate::cli;
use crate::cli::logo;

pub fn run(){
    logo::print();
    cli::hello();
}
