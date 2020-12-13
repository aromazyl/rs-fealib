//
// fe_extract.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//

use crate::config::{Config, FeaConfig};
use ::lru::LruCache;

pub trait FeaExtMethod {
    fn to_string(&self, tokens: &Vec<String>, config: &Config,
        fea_conf: &FeaConfig, cache: &mut LruCache<String, Vec<String>>) -> Result<Vec<u64>, String>;
}
