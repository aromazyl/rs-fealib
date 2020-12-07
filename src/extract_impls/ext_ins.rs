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
    fe_exts: Box<dyn FeaExtMethod>,
    fe_confs: Vec<FeaConfig>,
}

impl ExtIns {
    fn new(conf: &Config) -> ExtIns {
        let mut fe_confs: Vec<FeaConfig> = vec![];
        let mut fe_exts: Vec<Box<dyn FeaExtMethod>> = vec![];
        let f = std::fs::File::open(conf.feature_list_conf.clone()).unwrap();
        for line in BufReader::new(f).lines() {
            let line = line.unwrap();
            let fe_conf = FeaConfig::from_str(line.as_str()).unwrap();
            fe_confs.push(fe_conf);
            let method = match fe_conf.method.as_str() {
                "direct" => Box::new(DirectMethod),
                "combine" => Box::new(CombineMethod),
            };
        }
        let mut cache = LruCache::new(100);
        ExtIns {
            cache: cache,
            fe_confs: fe_confs,
        }
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
