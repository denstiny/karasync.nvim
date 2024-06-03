use std::{
    fs::{self, File},
    io::{self, Read, Result, Write},
    net::{Ipv4Addr, SocketAddr},
    path::Path,
    process::exit,
    str::FromStr,
};

use log::{error, info, warn};
use ssh2::Sftp;

use crate::logger::HandleResult;

/// 获取用户输入的ip地址和端口
#[allow(dead_code)]
pub fn input_parser_ssh() -> (String, u16) {
    let mut input = String::new();
    let mut ip = String::new();
    let mut port: u16 = 0;
    println!("Please enter the IP address");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            ip = input.trim().to_string();
            println!("ip: {}", ip);
            // 检查ip地址是否有效
            match Ipv4Addr::from_str(input.trim()) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("faild: {}", e);
                    exit(-1);
                }
            };
            input.clear();
        }
        Err(e) => eprintln!("input ip: {}", e),
    };
    println!("Please enter port");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            port = match input.trim().parse() {
                Ok(erp) => erp,
                Err(e) => {
                    println!("parse input port error: {}", e);
                    exit(-1);
                }
            };
            println!("port: {}", port);
            input.clear();
        }
        Err(e) => println!("input port: {}", e),
    };
    (ip, port)
}

#[allow(dead_code)]
pub fn get_user_cmd() -> Result<String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.trim().to_owned()),
        Err(e) => Err(e),
    }
}

pub fn exits_create(path: &str) {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path).unwrap()
    }
}

pub fn calculate_percentage(value: u32, total: u32) -> u32 {
    if total == 0 {
        return 100;
    }
    ((value as f64 / total as f64) * 100.0).round() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate() {
        let s = calculate_percentage(100, 100);
        println!("{}", s);
    }
}
