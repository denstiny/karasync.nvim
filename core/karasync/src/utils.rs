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

pub fn save_file(sftp: &Sftp, path: &Path, to_path: &str, is_dir: bool) -> String {
    //info!(
    //    "from: {}  to: {}  is_dir: {}",
    //    path.to_str().unwrap(),
    //    to_path,
    //    is_dir
    //);
    //let filename = path.file_name().unwrap().to_str().unwrap();
    let path = Path::new(path);
    if !is_dir {
        // TODO: 检查为什么读取失败
        let mut file = match sftp.open(path) {
            Ok(f) => f,
            Err(e) => return format!("faild: {}", e.message()),
        };
        let mut buf = vec![0; 1024];
        let mut local_file = match File::create(to_path) {
            Ok(f) => f,
            Err(err) => return format!("faild: create file {} => {}", to_path, err.to_string()),
        };
        while let Ok(n) = file.read(&mut buf) {
            if n == 0 {
                break;
            }
            match local_file.write_all(&buf[..n]) {
                Ok(_) => (),
                Err(_) => return format!("faild: write file {}", to_path),
            }
        }
    } else {
        exits_create(to_path);
    }
    format!(
        "sucessfully: downloaded file {}",
        path.file_name().unwrap().to_str().unwrap()
    )
}

pub fn calculate_percentage(value: u32, total: u32) -> u32 {
    if total == 0 {
        return 0;
    }
    ((value as f64 / total as f64) * 100.0).round() as u32
}
