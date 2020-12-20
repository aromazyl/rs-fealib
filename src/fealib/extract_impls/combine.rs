//
// combine.rs
// Copyright (C) 2020 zhangyule <zyl2336709@gmail.com>
// Distributed under terms of the MIT license.
//


use crate::config::{Config, FeaConfig};
use ::lru::LruCache;
use crate::fe_extract::FeaExtMethod;
use ::murmurhash64::murmur_hash64a;

pub struct CombineMethod;
impl FeaExtMethod for CombineMethod {
    fn encode(&self, tokens: &Vec<String>, config: &Config,
        fea_conf: &FeaConfig, cache: &mut LruCache<String, Vec<String>>) -> Result<Vec<u64>, String> {
        if tokens.len() != config.fe_tokens.len() {
            return Err("tokens length not equal to fe length".to_string());
        }
        let mut tmp: Vec<String> = Vec::with_capacity(20);
        for depends_name in &fea_conf.depends {
            let get_res = cache.get(depends_name);
            let cache_name = get_res.unwrap();
            if tmp.len() == 0 {
                for name in cache_name.iter() {
                    tmp.push(name.clone());
                }
                continue;
            }
            for token in cache_name.iter() {
                for x in tmp.iter_mut() {
                    *x = format!("{{{}}}-{{{}}}", x, token.as_str());
                }
            }
        }

        let res: Vec<u64> = tmp.iter().map(|x| murmur_hash64a(x.as_bytes(), 0) ^ (((fea_conf.slot_id as u64) << 32) as u64)).collect();
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
