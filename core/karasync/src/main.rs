#![allow(unused_imports)]
use ::log::info;
use core::panic;
use sshmanage::SSHManager;
mod logger;
mod rpc;
mod utils;

static LOG_PATH: &str = "log.txt";

fn main() {
    logger::init_logger(LOG_PATH.to_owned());
    info!("karasync start");
    rpc::await_accept("127.0.0.1:5555");
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
