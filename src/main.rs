#![allow(non_snake_case)]
#![allow(unused)]

use clap::Parser;
use clap::{Arg, App, ArgGroup};
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli{
    #[clap(short, long)]
    pattern: String,

    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

struct TodoList{
    name: String,
    items: Vec<String>
}

impl TodoList{
    fn new(newName: String, newItems: Vec<String>) -> Self {
        TodoList {name: newName, items: newItems}
    }
}

fn main() {
    //let args = Cli::parse();
    let app = App::new("todo")
        .version("1.0")
        .about("Store daily todo list")
        .author("Sridhar Nandigam");

    let name_option = Arg::new("name")
        .long("name")
        .takes_value(true)
        .help("Who to say hello to");

    let add_item = Arg::new("additem")
        .long("additem")
        .takes_value(true)
        .short('a');

    let app = app.arg(name_option)
                .arg(add_item)
                .group(ArgGroup::new("options")
                    .args(&["name", "additem"])
                    .required(true));

    let matches = app.get_matches();
        
    if(matches.is_present("name")){
        let name = matches.value_of("name").unwrap();
        println!("Got name: {name}")
    } else if(matches.is_present("additem")){
        let item = matches.value_of("additem").unwrap();
        println!("Received item: {item}");
    } else{
        println!("Bruh.....");
    }
}
