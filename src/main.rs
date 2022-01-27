#![allow(non_snake_case)]
#![allow(unused)]

use clap::Parser;
use clap::{Arg, App, ArgGroup};
use std::env;
use std::collections::HashMap;

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
    items: HashMap<String, bool>
}

impl TodoList{
    fn new(newName: String) -> Self {
        TodoList {name: newName, items: HashMap::new()}
    }
}

fn main() {
    //let args = Cli::parse();
    let app = App::new("todo")
        .version("1.0")
        .about("Store daily todo list")
        .author("Sridhar Nandigam");

    let new_list = Arg::new("newlist")
        .long("newlist")
        .short('n')
        .takes_value(true)
        .help("Who to say hello to");

    let add_item = Arg::new("additem")
        .multiple_values(true)
        .long("additem")
        .takes_value(true)
        .short('a')
        .help("Add item to list");
    
    

    let app = app.arg(new_list)
                .arg(add_item)
                .group(ArgGroup::new("options")
                    .args(&["newlist", "additem"])
                    .required(true));

    let matches = app.get_matches();
    
    if(matches.is_present("newlist")){
        let list_name = matches.value_of("newlist").unwrap();
        let new_todo = TodoList::new(list_name.to_string());
        println!("Got name: {list_name}")
    } else if(matches.is_present("additem")){
        let items: Vec<&str> = matches.values_of("additem").unwrap().collect();
        println!("Received item: {:?}", items);
    } else{
        println!("Bruh.....");
    }
}
