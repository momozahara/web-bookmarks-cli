use dirs::home_dir;
use std::{ fs, path::PathBuf };

pub fn pwd() -> PathBuf {
    home_dir()
        .and_then(|a| Some(a.join(r"pcode-cli\open\data.json")))
        .unwrap()
}

pub fn load() -> String {
    let target_path = home_dir()
        .and_then(|a| Some(a.join(r"pcode-cli\open\data.json")))
        .unwrap();
    let path = target_path.as_path();

    match &fs::read_to_string(path) {
        Ok(r) => r.clone(),
        Err(_) => String::from("[]"),
    }
}

pub fn create(data: String) {
    let target_path = home_dir()
        .and_then(|a| Some(a.join(r"pcode-cli\open\data.json")))
        .unwrap();
    let path = target_path.as_path();
    let prefix = path.parent().unwrap();

    fs::create_dir_all(prefix).unwrap();
    fs::write(path, data).unwrap();
}