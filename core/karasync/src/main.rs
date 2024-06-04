#![allow(unused_imports)]
use crate::config::{get_config, parse_config};
use ::log::info;
mod config;
mod logger;
mod rpc;
mod utils;

fn main() {
    //  解析启动配置
    parse_config();
    rpc::await_accept(get_config().host().as_str());
}
