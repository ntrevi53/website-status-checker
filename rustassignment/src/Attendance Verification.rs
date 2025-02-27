use std::fs::File;
use std::io::prelude::*;

struct Config {
    student_name: String,
    s_id: String,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let student_name = lines.next().unwrap().to_string();
        let s_id = lines.next().unwrap().to_string();

        Config { student_name, s_id}
    }
}

fn reading_from_file() {
    let config = Config::from_file("my_files/config.txt");
    println!("Name: {}", config.student_name);
    println!("ID: {}", config.s_id);
}

fn main() {
    reading_from_file();
}