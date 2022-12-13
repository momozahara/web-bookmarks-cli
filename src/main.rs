mod cli;
mod file;

use pcre2::bytes::Regex;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
struct Data {
    name: String,
    target: String,
}

fn main() {
    let matches = cli::init().get_matches();

    match matches.get_one::<String>("open") {
        Some(name) => {
            let mut target: String = String::from("");

            let mut objs: Vec<Data> = serde_json::from_str(file::load().as_str()).unwrap();

            for o in objs.iter_mut() {
                if o.name == name.clone() {
                    target = o.target.clone();
                }
            }

            if target.is_empty() {
                println!("{} not found!", name);
            } else {
                webbrowser::open(target.clone().as_str()).unwrap();
            }

            return ();
        }
        None => (),
    }

    match matches.subcommand() {
        Some(("pwd", _)) => {
            println!("{}", file::pwd().as_path().display());
        }
        Some(("ls", _)) => {
            let objs: Vec<Data> = serde_json::from_str(file::load().as_str()).unwrap();
            for o in objs.iter().clone() {
                println!("{} {}", o.name, o.target);
            }
            println!("{} found.", objs.len());
        }
        Some(("add", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap().clone();
            let target = sub_matches.get_one::<String>("target").unwrap().clone();

            match
                Regex::new(
                    r"(http[s]?):\/\/[-a-zA-Z0-9@:%_\+.~#?&//=]{2,256}\.[a-z]{2,4}\b(\/[-a-zA-Z0-9@:%_\+.~#?&//=]*)?"
                )
                    .unwrap()
                    .is_match(target.as_bytes())
                    .unwrap()
            {
                false =>
                    panic!("wrong target given {} required format (http(s)://www.google.com | http(s)://google.com)", target),
                true => {}
            }

            let mut objs: Vec<Data> = serde_json::from_str(file::load().as_str()).unwrap();

            let mut existed = false;
            for o in objs.iter_mut() {
                if o.name == name.clone() {
                    o.target = target.clone().into();
                    existed = true;
                }
            }

            if !existed {
                objs.push(Data {
                    name: name.clone().into(),
                    target: target.clone().into(),
                });
            }

            println!("{} {} added.", name, target);
            file::create(serde_json::to_string(&objs).unwrap());
        }
        Some(("remove", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name").unwrap().clone();

            let mut objs: Vec<Data> = serde_json::from_str(file::load().as_str()).unwrap();

            let mut existed = false;
            let mut index = 0;
            for (i, o) in objs.iter().enumerate() {
                if o.name == name.clone() {
                    existed = true;
                    index = i;
                }
            }

            if existed {
                println!("{} removed.", name);
                objs.remove(index);
                file::create(serde_json::to_string(&objs).unwrap());
            } else {
                println!("{} not found!", name);
            }
        }
        _ => unreachable!(),
    }
}