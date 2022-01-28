#![allow(non_snake_case)]
#![allow(unused)]

use clap::Parser;
use clap::{Arg, App, ArgGroup};
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::ErrorKind;
use chrono::{DateTime, Local, NaiveDate, Utc};

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
    date: chrono::DateTime<chrono::Utc>
}

impl TodoList{
    fn new(new_name: String) -> Self {
        TodoList {name: new_name, items: HashMap::new(), date: chrono::Utc::now()}
    }

    fn load(new_name: String, new_date: String) -> Self{
        let naive = NaiveDate::parse_from_str(&new_date, &DATE_FORMAT).unwrap().and_hms(0, 0, 0);
        TodoList {name: new_name, items: HashMap::new(), date: DateTime::<Utc>::from_utc(naive, Utc)}
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
        content.push_str("\n");

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
    let mut newline = "";
    for x in input{
        let record = x.save();
        output.push_str(&record);
        output.push_str("--");
        output.push_str(newline);
        newline = "\n"
    }

    //std::fs::write("cache.txt", output)
    let mut file = File::create("cache.txt")?;
    file.write(output.as_bytes());
    Ok(())
}

fn retrieve_from_file() -> Result<Vec<TodoList>, std::io::Error>{
    let mut f = File::open("cache.txt");

    let mut data = String::new();
    let mut return_vec: Vec<TodoList> = Vec::new();

    let mut f = match f{
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => return Ok(return_vec),
            other_error => {
                panic!("Something went wrong when returning empty vector: {:?}", other_error);
            }
        },
    };

    f.read_to_string(&mut data)?;

    let sections = data.split_terminator("--\n");

    for s in sections{
        let mut line_iter = s.lines();
        let name = line_iter.next();
        let date = line_iter.next();

        println!("New Section");
        println!("Section: {:?}", s);
        println!("Name: {:?}, Date: {:?}", name, date);

        let mut temp_todo = TodoList::load(name.unwrap().to_string(), date.unwrap().to_string());

        for line in line_iter{
            temp_todo.additem(line.to_string());
        }
        return_vec.push(temp_todo);
    }

    println!("Debug input");
    
    for x in &return_vec{
        x.print();
    }
    Ok(return_vec)
}

fn main() {
    //let args = Cli::parse();
    let mut list_vec: Vec<TodoList> = retrieve_from_file().expect("Error while creating vector");

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
