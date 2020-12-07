//
// ext_ins.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//

use std::str::FromStr;
use crate::fe_extract::FeaExtMethod;
use crate::config::{Config, FeaConfig};
use crate::combine::CombineMethod;
use crate::direct::DirectMethod;
use ::lru::LruCache;
use std::io::BufReader;
use std::io::BufRead;

struct ExtIns {
    cache: LruCache<String, Vec<String>>,
    fe_exts: Vec<Box<FeaExtMethod>>,
    fe_confs: Vec<FeaConfig>,
}

impl ExtIns {
    fn new(conf: &Config) -> ExtIns {
        let mut fe_confs: Vec<FeaConfig> = vec![];
        let mut fe_exts: Vec<Box<FeaExtMethod>> = vec![];
        let f = std::fs::File::open(conf.feature_list_conf.clone()).unwrap();
        for line in BufReader::new(f).lines() {
            let line = line.unwrap();
            let fe_conf = FeaConfig::from_str(line.as_str()).unwrap();
            let method = fe_conf.method.clone();
            fe_confs.push(fe_conf);
            match method.as_str() {
                "direct" => fe_exts.push(Box::new(DirectMethod)),
                "combine" => fe_exts.push(Box::new(CombineMethod)),
                x => panic!(format!("error format {}", x)),
            };
        }
        let mut cache = LruCache::new(100);
        ExtIns {
            cache: cache,
            fe_exts: fe_exts,
            fe_confs: fe_confs,
        }
    }

    fn compute(&mut self, ins: &Vec<String>, conf: &Config, buf: &'static mut String) -> bool {
        buf.clear();
        // show click
        buf.push_str(ins[ins.len()-1].as_str());
        buf.push(' ');

        for (index, method) in self.fe_exts.iter().enumerate() {
            if let Ok(coded_string) = method.to_string(&ins, &conf, &self.fe_confs[index], &mut self.cache)  {
                if self.fe_confs[index].is_output {
                    for code in coded_string {
                        buf.push_str(code.to_string().as_str());
                        buf.push(':');
                        buf.push_str(self.fe_confs[index].slot_id.to_string().as_str());
                        buf.push(' ');
                    }
                }
            } else {
                return false;
            }
        }
        for addtoken in &conf.add_tokens {
            if let Some(idx) = conf.fe_tokens.iter().position(|x| x == addtoken) {
                buf.push('\t');
                buf.push_str(ins[idx].as_str());
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
