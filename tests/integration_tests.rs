extern crate rs_fealib;
use rs_fealib::config::{Config,FeaConfig};
use rs_fealib::ext_ins::{ExtIns};

#[cfg(test)]
mod tests {
    use super::*;
    fn it_works() {
        let config = Config::new("./test.yaml".to_string()).unwrap();
        let mut ext_ins_helper = ExtIns::new(&config);
        let mut vec_buf: Vec<String> = "1\t2\t3\t4\t5\t6\t1 0".split('\t').into_iter().map(|x|x.to_string()).collect::<Vec<String>>();
        let mut str_buf = String::new();
        let res = ext_ins_helper.compute(&vec_buf, &config, &mut str_buf);
        assert_eq!(res, true);
        println!("test result: [[{}]]", str_buf);
    }
}