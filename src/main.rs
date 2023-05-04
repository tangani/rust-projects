//-- ##############################
//-- Task: Implementing get request
//-- Author: Tangani Moyo
//-- Version: 1.0.0
//-- Date: 03 April 23
//-- ##############################

mod ip_locator;

fn main() {

    let country = ip_locator::main();
    println!("{}", country);
}