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

    info!("karasync start");
    rpc::await_accept(get_config().host().as_str());
    info!("karasync exit");

    //let (ip, port) = utils::input_parser_ssh();
    //let mut ssh = SSHManager::new(&ip[..], port);
    //match ssh.userauth_password("root", "asd") {
    //    true => {
    //        println!("🔗 connected ok");
    //    }
    //    false => panic!("🔗 Failed to connect to host"),
    //};

    //loop {
    //    let cmd = utils::get_user_cmd();
    //    match ssh.exec(cmd.expect("ls")) {
    //        Ok(result) => println!("{}", result),
    //        Err(e) => println!("{}", e.msg),
    //    }
    //}
}
