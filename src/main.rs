#![allow(non_snake_case)]
#![allow(unused)]

use clap::Parser;
use clap::{Arg, App, ArgGroup};
use std::env;
use std::collections::HashMap;
use chrono;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

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
    items: HashMap<String, bool>,
    date: chrono::DateTime<chrono::Local>
}

impl TodoList{
    fn new(newName: String) -> Self {
        TodoList {name: newName, items: HashMap::new(), date: chrono::offset::Local::now()}
    }

    fn additem(&mut self, newItem: String){
        self.items.insert(newItem, false);
    }

    fn print(&self){
        println!("{} - {}", self.name, self.date.format(&DATE_FORMAT).to_string());
    }

    fn save(&self) -> String{
        let mut content = String::new();

        content.push_str(&self.name);
        content.push_str("\n");
        content.push_str(&self.date.format(&DATE_FORMAT).to_string());

        for (k, v) in &self.items{
            let record = format!("{}\t{}\n", k, v);
            content.push_str(&record)
        }
        println!("{}", content);
        content
    }
}

fn save_to_file(input: &Vec<TodoList>) -> Result<(), std::io::Error>{
    let mut output = String::new();
    for x in input{
        let record = x.save();
        output.push_str(&record);
        output.push_str("--");
    }

    std::fs::write("cache.txt", output)
}

fn main() {
    //let args = Cli::parse();
    let mut list_vec: Vec<TodoList> = Vec::new();

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

    let view_all = Arg::new("viewall")
        .long("viewall")
        .takes_value(false)
        .short('v')
        .help("View all lists");
    
    

    let app = app.arg(new_list)
                .arg(add_item)
                .arg(view_all)
                .group(ArgGroup::new("options")
                    .args(&["newlist", "additem", "viewall"])
                    .required(true));

    let matches = app.get_matches();
    
    if(matches.is_present("newlist")){
        let list_name = matches.value_of("newlist").unwrap();
        let mut new_todo = TodoList::new(list_name.to_string());

        list_vec.push(new_todo);

        for x in &list_vec {
            x.save();
        }
        println!("{}", list_vec.len())
    } else if(matches.is_present("additem")){
        let items: Vec<&str> = matches.values_of("additem").unwrap().collect();
        println!("Received item: {:?}", items);
    } else if(matches.is_present("viewall")){
        if(list_vec.len() == 0){
            println!("No lists present")
        } else{
            for x in &list_vec {
                x.print();
            }
        }
    } else{
        println!("Bruh, how tf.....");
    }

    save_to_file(&list_vec);
}
