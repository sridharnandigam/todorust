#![allow(non_snake_case)]
#![allow(unused)]

use clap::Parser;
use std::env;
/*
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli{
    #[clap(short, long)]
    pattern: String,

    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
*/

fn main() {
    //add task
    //remove task
    //edit task
    //complete task
    //list tasks
    //help (list commands)
    //let args = Cli::parse();

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
}
