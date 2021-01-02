mod engine;
use engine::Engine;
use crate::feature::{MulFeaDef, FeaDef, MulScore, Score};
use crate::config::{Config};
use std::io::{self, BufRead};
use ::dubble::DoubleBuffered;
use crate::ext_ins::ExtIns;
use std::collections::HashMap;
extern crate log;
use log::{info, trace, warn};


#[derive(Clone)]
pub struct LrEngine {
	extor : ExtIns,
    fea_weighteds : DoubleBuffered<HashMap<u64, f32>>,
    cur_version : String,
}

impl Engine for LrEngine {
    // fn new(config: &Config) -> Self {
    //     LrEngine {
    //         extor: ExtIns::new(config),
    //         fea_weighteds: DoubleBuffered::construct_with(|| HashMap<u64,f32>::new()),
    //         cur_version: "".to_string(),
    //         compute_buf: Vec::<String>::new(),
    //     }
    // }

	fn load_model(&mut self, conf: &String, version: &String) {
        if self.cur_version == version.clone() {
            return;
        }
        self.cur_version = version.clone();
        let f = std::fs::File::open(conf).unwrap();
        let mut vec_buf: Vec<String> = vec![];
        self.fea_weighteds.clear();
		for line in io::BufReader::new(f).lines() {
			let line = line.unwrap();
			if line.trim().len() == 0 {
				continue;
			}
            vec_buf = line.split("\t").into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
            self.fea_weighteds.insert(vec_buf[0].parse::<u64>().unwrap(),
                    vec_buf[1].parse::<f32>().unwrap());
        }
        self.fea_weighteds.update();
    }

    fn predict(&mut self, ins: &[MulFeaDef], sid: &String) -> MulScore {
        let mut scores = MulScore::new();
        let mut fea_vec = HashMap::<String, String>::new();
        let mut hash_res = HashMap::<i32, Vec<u64>>::new();
        for mul_feas in ins {
            fea_vec.clear();
            for fea_def in mul_feas.get_features() {
                fea_vec.insert(fea_def.get_name().to_string(), fea_def.get_value().to_string());
            }
            hash_res.clear();
            let mut score: f32 = 0.0f32;
            if self.extor.hash(&fea_vec, &mut hash_res) {
                for value in hash_res.values() {
                    for sign in value {
                        if let Some(weight) = self.fea_weighteds.get(sign) {
                            score += weight;
                        }
                    }
                }
                if (score > 0.0f32) {
                    score = 1.0f32 / (1.0f32+(-score).exp());
                } else {
                    score = score.exp() / (1.0f32+score.exp());
                }
            } else {
                //warn!("unable to extract ins sid = {}, feature {:?}", sid, mul_feas);
            }
            let mut _score = Score::new();
            _score.set_val(score);
            scores.mut_score().push(_score);
        }
        scores
    }
    
}
