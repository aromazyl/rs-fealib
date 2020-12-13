//
// config.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//
//
use std::default::Default;
use serde_yaml;
use ::serde::{Serialize, Deserialize};
use std::str::FromStr;
use ::regex::Regex;

#[derive(Debug, Serialize, Deserialize)]
pub struct FeaConfig {
    pub name: String,

    pub slot_id: i32,

    pub is_output: bool,

    pub method: String,

    pub depends: Vec<String>,

    pub args: String,
}

impl Default for FeaConfig {
    fn default() -> Self {
        FeaConfig {
            name: "".to_string(),
            slot_id: 0,
            is_output: true,
            method: "direct".to_string(),
            depends: vec![],
            args: "".to_string(),
        }
    }
}

// name=xx;slot_id=xx;is_output=xx;method=xx;depends=xx;args=xx
//
impl FromStr for FeaConfig {
    type Err = String;
    fn from_str(fea_conf: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new("\
            ^name=(?P<name>[^;]*);\
            slot_id=(?P<slot_id>[[:digit:]]+);\
            is_output=(?P<is_output>true|false);\
            method=(?P<method>[[:word:]]+);\
            depends=(?P<depends>[^;]*);\
            args=(?P<args>[^;]*)").unwrap();
        }
        RE.captures(fea_conf).map(|caps| {
            FeaConfig {
                name: caps.name("name").unwrap().as_str().to_string(),
                slot_id: caps.name("slot_id").unwrap().as_str().parse::<i32>().unwrap(),
                is_output: caps.name("is_output").unwrap().as_str().parse::<bool>().unwrap(),
                method: caps.name("method").unwrap().as_str().to_string(),
                depends: caps.name("depends").unwrap().as_str().split(",").map(|x| x.to_string()).collect::<Vec<String>>(),
                args: caps.name("args").unwrap().as_str().to_string(),
            }
        }).ok_or_else(|| fea_conf.to_string().clone())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub cache_size: u32,

    pub fe_tokens: Vec<String>,

    pub add_tokens: Vec<String>,

    pub feature_list_conf: String,

}

impl Default for Config {
    fn default() -> Self {
        Config {
            cache_size: 0,
            fe_tokens: vec![],
            add_tokens: vec![],
            feature_list_conf: "".to_string(),
        }
    }
}

impl Config {
    pub fn new(filename: String) -> Result<Config, serde_yaml::Error> {
        let f = std::fs::File::open(filename).unwrap();
        serde_yaml::from_reader(f)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let ref test_str1 = "name=A;slot_id=10;is_output=true;method=direct;depends=C;args=0";
        let test_conf = test_str1.parse::<FeaConfig>().unwrap();
        assert_eq!(test_conf.name, "A".to_string());
        assert_eq!(test_conf.slot_id, 10);
        assert_eq!(test_conf.is_output, true);
        assert_eq!(test_conf.method, "direct".to_string());
        assert_eq!(test_conf.depends, vec!["C".to_string()]);
        assert_eq!(test_conf.args, "0".to_string());
        let config = Config::new("test.yaml".to_string()).unwrap();
        assert_eq!(config.cache_size, 100);
        assert_eq!(config.fe_tokens, vec!["a", "b", "c", "d", "e", "f", "show_click"].iter().map(|x| x.to_string()).collect::<Vec<String>>());
    }
}
