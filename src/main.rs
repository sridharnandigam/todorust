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
use std::path::Path;
use std::fs::OpenOptions;
use chrono::{DateTime, Local, NaiveDateTime, Utc};

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";
const CACHE_PATH: &str = "/home/potato/Sridhar/random/cache.txt";

struct TodoList{
    name: String,
    items: Vec<(String, bool)>,
    date: chrono::DateTime<chrono::Utc>
}

impl TodoList{
    fn new(new_name: &String) -> Self {
        TodoList {name: new_name.to_string(), items: Vec::new(), date: chrono::Utc::now()}
    }

    fn load(new_name: String, new_date: String) -> Self{
        let naive = NaiveDateTime::parse_from_str(&new_date, &DATE_FORMAT).unwrap();
        TodoList {name: new_name, items: Vec::new(), date: DateTime::<Utc>::from_utc(naive, Utc)}
    }

    fn loaditem(&mut self, line: String){
        let v = line.splitn(2, "\t").collect::<Vec<&str>>();
        //println!("Loading item: {:?}", v);
        &self.items.push((v[0].to_string(), std::str::FromStr::from_str(v[1]).unwrap()));
    }

    fn additem(&mut self, newItem: String){
        self.items.push((newItem, false));
    }

    fn print(&self){
        println!("{} - {}", self.name, self.date.format(&DATE_FORMAT).to_string());
        for (i, (key, value)) in self.items.iter().enumerate() {
            println!("\t[{}] {} {}", i, key, value);
        }
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
        output.push_str("\n");
    }

    //std::fs::write("cache.txt", output)
    
    let mut file = OpenOptions::new().write(true).truncate(true).create(true).open(CACHE_PATH);

    let mut file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error encountered: {:?}", error),
    };

    //println!("Output: {:?}", output);


    write!(&mut file, "{}", output);

    Ok(())
}

fn retrieve_from_file() -> Result<Vec<TodoList>, std::io::Error>{
    let mut f = File::open(CACHE_PATH);

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

        let mut temp_todo = TodoList::load(name.unwrap().to_string(), date.unwrap().to_string());

        for line in line_iter{
            temp_todo.loaditem(line.to_string());
        }
        return_vec.push(temp_todo);
    }
    Ok(return_vec)
}

fn main() {
    //let args = Cli::parse();
    let mut list_vec: Vec<TodoList> = retrieve_from_file().expect("Error while creating vector");

    let app = App::new("todo")
                .version("1.0")
                .about("Store daily todo list")
                .author("Sridhar Nandigam")
                .subcommand(App::new("new")
                            .arg(Arg::new("name")
                                .long("name")
                                .short('n')
                                .takes_value(true)
                                .help("name of new list"))
                            .about("Create new list"))
                .subcommand(App::new("add")
                            .arg(Arg::new("list")
                                .long("list")
                                .short('l')
                                .takes_value(true)
                                .help("provide list index"))
                            .arg(Arg::new("item")
                                .long("item")
                                .short('i')
                                .takes_value(true)
                                .help("provide item index"))
                            .about("Add item to existing list"))
                .subcommand(App::new("complete")
                            .arg(Arg::new("list")
                                .long("list")
                                .short('l')
                                .takes_value(true)
                                .help("provide list index"))
                            .arg(Arg::new("item")
                                .long("item")
                                .short('i')
                                .takes_value(true)
                                .help("provide item index"))
                            .about("Mark item as completed on existing list"))
                .subcommand(App::new("view")
                            .arg(Arg::new("list")
                                .long("list")
                                .short('l')
                                .takes_value(true)
                                .help("provide list index"))
                            .about("View specific list"))   
                .subcommand(App::new("all")
                            .about("View all lists"))
                .subcommand(App::new("rmitem")
                            .arg(Arg::new("list")
                                .long("list")
                                .short('l')
                                .takes_value(true)
                                .help("provide list index"))
                            .arg(Arg::new("item")
                                .long("item")
                                .short('i')
                                .takes_value(true)
                                .help("provide item index"))
                            .about("Remove an item"))
                .subcommand(App::new("undo")
                            .arg(Arg::new("list")
                                .long("list")
                                .short('l')
                                .takes_value(true)
                                .help("provide list index"))
                            .arg(Arg::new("item")
                                .long("item")
                                .short('i')
                                .takes_value(true)
                                .help("provide item index"))
                            .about("Reset task as incomplete"))
                .subcommand(App::new("throw")
                            .arg(Arg::new("list")
                                .long("list")
                                .short('l')
                                .takes_value(true)
                                .help("provide list index"))
                            .about("Delete todo list"));

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let list_name = sub_m.value_of("name").expect("MISSING ARG NAME");
            let mut new_todo = TodoList::new(&list_name.to_string());

            list_vec.push(new_todo);
        },
        Some(("throw", sub_m)) => {
            let index: usize = sub_m.value_of_t("list").expect("MISSING ARG LIST");

            assert!(index < list_vec.len(), "List index out of bounds");
            list_vec.remove(index);
        }
        Some(("add", sub_m)) => {
            let index: usize = sub_m.value_of_t("list").expect("MISSING ARG LIST");
            let item = sub_m.value_of("item").expect("MISSING ARG ITEM");

            assert!(index < list_vec.len(), "List index out of bounds");
            list_vec[index].additem(item.to_string());
            list_vec[index].print();
        },
        Some(("complete", sub_m)) => {
            let index: usize = sub_m.value_of_t("list").expect("MISSING ARG LIST");
            let item: usize = sub_m.value_of_t("item").expect("MISSING ARG ITEM");

            assert!(index < list_vec.len(), "List index out of bounds");
            assert!(item < list_vec[index].items.len(), "Item index out of bounds");
            list_vec[index].items[item].1 = true;
            list_vec[index].print();
        },
        Some(("undo", sub_m)) => {
            let index: usize = sub_m.value_of_t("list").expect("MISSING ARG LIST");
            let item: usize = sub_m.value_of_t("item").expect("MISSING ARG ITEM");

            assert!(index < list_vec.len(), "List index out of bounds");
            assert!(item < list_vec[index].items.len(), "Item index out of bounds");
            list_vec[index].items[item].1 = false;
            list_vec[index].print();
        },
        Some(("view", sub_m)) => {
            let index: usize = sub_m.value_of_t("list").expect("MISSING ARG LIST");

            assert!(index < list_vec.len(), "List index out of bounds");
            print!("[{}] ", index);
            list_vec[index].print();
        }
        Some(("all", sub_m)) => {
            if(list_vec.len() == 0){
                println!("No lists present")
            } else{
                for (i, x) in list_vec.iter().enumerate() {
                    print!("[{}] ", i);
                    x.print();
                }
            }
        },
        Some(("rmitem", sub_m)) => {
            let index: usize = sub_m.value_of_t("list").expect("MISSING ARG LIST");
            let item: usize = sub_m.value_of_t("item").expect("MISSING ARG ITEM");

            assert!(index < list_vec.len(), "List index out of bounds");
            assert!(item < list_vec[index].items.len(), "Item index out of bounds");
            list_vec[index].items.remove(item);
            list_vec[index].print();
        }
        _ => {
            println!("No input detected");
        }
    }
    save_to_file(&list_vec);
}
