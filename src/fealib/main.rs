extern crate rs_fealib;
use rs_fealib::config::{Config};
use rs_fealib::ext_ins::{ExtIns};
extern crate clap;
use clap::{Arg, App};
use std::io::{self, BufRead};

fn main() {
    let matches = App::new("Rs-Fealib")
                       .version("1.0")
                       .author("aromazyl")
                       .about("feature extraction library in rust")
                       .arg(Arg::with_name("config")
                               .short("c")
                               .long("config")
                               .value_name("FILE")
                               .help("Sets the feature extraction yaml configuration")
                               .required(true)).get_matches();

    let yaml_config = matches.value_of("config").unwrap_or("test.yaml").to_string();
    let config = Config::new(yaml_config).unwrap();
    let mut ext_ins_helper = ExtIns::new(&config);
    let mut vec_buf: Vec<String>;
    let mut str_buf = String::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line.trim().len() == 0 {
            continue;
        }
        vec_buf = line.split("\t").into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
        let res = ext_ins_helper.compute(&vec_buf, &config, &mut str_buf);
        if res == true {
            println!("{}", str_buf);
        }
    }
}
