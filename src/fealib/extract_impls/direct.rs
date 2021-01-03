//
// direct.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//

use crate::config::{Config, FeaConfig};
use crate::fe_extract::FeaExtMethod;
use ::murmurhash64::murmur_hash64a;
use std::collections::HashMap;

pub struct DirectMethod;
impl FeaExtMethod for DirectMethod {
    fn encode(&self, tokens: &Vec<String>, config: &Config,
        fea_conf: &FeaConfig, cache: &mut HashMap<String, Vec<String>>) -> Result<Vec<u64>, String> {
        if tokens.len() != config.fe_tokens.len() {
            return Err("tokens length not equal to fe length".to_string());
        }
        let idx = config.fe_tokens.iter().position(|x| x == &fea_conf.depends[0]).unwrap();
        cache.insert(fea_conf.name.clone(), vec![tokens[idx].to_string()]);
        if tokens[idx].as_str() == "-" || fea_conf.is_output == false {
            return Ok(vec![])
        }
        Ok(vec![murmur_hash64a(tokens[idx].as_bytes(), 0) ^ (((fea_conf.slot_id as u64) << 32) as u64)])
    }
    fn hash(&self, tokens: &HashMap<String, String>, fea_conf: &FeaConfig, cache: &mut HashMap<String, Vec<String>>) -> Result<Vec<u64>, String> {
            let value = tokens.get(&fea_conf.depends[0]).unwrap();
            cache.insert(fea_conf.name.clone(), vec![value.clone()]);
        if value == "-" || fea_conf.is_output == false {
                return Ok(vec![])
            }
            Ok(vec![murmur_hash64a(value.as_bytes(), 0) ^ (((fea_conf.slot_id as u64) << 32) as u64)])
        }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
