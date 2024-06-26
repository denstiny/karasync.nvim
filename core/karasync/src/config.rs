use std::{env, sync::Mutex};

use lazy_static::lazy_static;
use log::info;

use crate::{logger, utils::exits_create};

#[allow(dead_code)]
pub struct Config {
    pub data_dir: String,
    pub ip: String,
    pub port: u16,
    pub email: String,
    pub name: String,
}

impl Config {
    pub fn host(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config {
        data_dir: String::new(),
        ip: String::new(),
        port: u16::MIN,
        email: String::new(),
        name: String::new()
    });
}

pub fn parse_config() {
    let args: Vec<String> = env::args().skip(1).collect();
    let data_dir = args.get(0).unwrap();
    let ip = args.get(1).unwrap();
    let port: u16 = args.get(2).unwrap().parse().unwrap();
    let name = args.get(3).unwrap();
    let email = args.get(4).unwrap();

    let mut config = CONFIG.lock().unwrap();
    config.ip = ip.clone();
    config.data_dir = data_dir.clone();
    config.port = port;
    config.name = name.clone();
    config.email = email.clone();
    // 检查目录运行要求的数据目录
    exits_create(&config.data_dir);

    // 初始化日志

    let log_file = format!("{}/karasync.log", config.data_dir);
    logger::init_logger(log_file);
}

pub fn get_config() -> Config {
    let config = CONFIG.lock().unwrap();
    Config {
        data_dir: config.data_dir.clone(),
        ip: config.ip.clone(),
        port: config.port,
        name: config.name.clone(),
        email: config.email.clone(),
    }
}
