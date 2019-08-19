#[macro_use]
extern crate clap;

use clap::App;
use std::fs::File;
use std::io::Read;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    if matches.is_present("FILES") {
        let file_list: Vec<_> = matches.values_of("FILES").unwrap().collect();

        for file in file_list {
            let mut file = File::open(file).unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            convert_to_uwu( buf.as_str());
        }
    } else {
        loop {
            let mut buf = String::new();
            let bytes = std::io::stdin().read_line(&mut buf).unwrap();
            if bytes == 0 {
                break;
            }
            convert_to_uwu( buf.as_str());
        }
    }

}

fn convert_to_uwu(uwu_str: &str) {
    let mut uwu_string = uwu_str.to_string();
    uwu_string = uwu_string
        .replace("r", "w")
        .replace("l", "w")
        .replace("ask", "awsk");
    println!("{}", uwu_string);
}
