//
// direct.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//

use crate::config::{Config, FeaConfig};
use ::lru::LruCache;
use crate::fe_extract::FeaExtMethod;
use ::murmurhash64::murmur_hash64a;

pub struct DirectMethod;
impl FeaExtMethod for DirectMethod {
    fn to_string(&self, tokens: &Vec<String>, config: &Config,
        fea_conf: &FeaConfig, cache: &mut LruCache<String, Vec<String>>) -> Result<Vec<u64>, String> {
        if tokens.len() != config.fe_tokens.len() {
            return Err("tokens length not equal to fe length".to_string());
        }
        let idx = config.fe_tokens.iter().position(|x| x == &fea_conf.depends[0]).unwrap();
        cache.put(fea_conf.name.clone(), vec![tokens[idx].to_string()]);
        if tokens[idx].as_str() == "-" {
            return Ok(vec![])
        }
        Ok(vec![murmur_hash64a(tokens[idx].as_bytes(), 0) ^ (((fea_conf.slot_id as u64) << 32) as u64)])
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
