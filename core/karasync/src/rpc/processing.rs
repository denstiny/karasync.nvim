use super::fast::{self, ssh};
use super::structs::{AsyncTaskMsg, DirInfo, ReprMessage, ReprMessageMsg};
//use super::ArcQueue;
use crate::config::get_config;
use crate::logger::HandleResult;
use crate::rpc::structs::{AsyncGitClone, AsyncTask};
use crate::rpc::structs::{Message, MessageCode};
use crate::utils::{calculate_percentage, exits_create, save_file};
use lazy_static::lazy_static;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ssh2::{FileStat, Session};
use sshmanage::SSHManager;
use std::collections::{HashMap, VecDeque};
use std::fmt::format;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::sync::Mutex as stdMutex;
use std::sync::{mpsc, Arc};
use std::{io::Write, net::TcpStream};
use tokio::sync::{Mutex, Notify};

// 发布消息
fn sub_nofity(sender: &mpsc::Sender<String>, result: String) {
    match sender.send(result.clone()) {
        Ok(_) => info!("sender_send: {}", { result }),
        Err(err) => {
            warn!("sender_faild: {}", err.to_string())
        }
    };
}

// 克隆项目到本地
pub fn async_project_clone(data: Value, sender: &mpsc::Sender<String>) -> String {
    // 将收到的消息转换成本地接口
    let task: AsyncTask<AsyncGitClone> = match serde_json::from_value(data) {
        Ok(data) => data,
        Err(e) => {
            return format!("Faile: json to AsyncTask<AsyncGitClone> {}", e);
        }
    };
    let user = task.msg.user;
    let password = task.msg.password;
    let code = task.code;
    let id = task.id.as_str();
    let data_dir = get_config().data_dir;

    // 检查host是否正确
    let socket_addr: SocketAddr = match task.msg.host.parse() {
        Ok(addr) => addr,
        Err(_) => return repr_message(id, code, "faild: input host is faild", 100),
    };

    // 登录到远程服务器
    let (ip, port) = (socket_addr.ip().to_string(), socket_addr.port());
    let ssh: Session = match fast::ssh::new(ip, port, user, password) {
        Ok(ssh) => ssh,
        Err(e) => return repr_message(id, code, e.to_string().as_str(), 100),
    };

    let path = Path::new(task.msg.path.as_str());
    let sftp = ssh.sftp().unwrap();

    // 获取远程目录下的所有文件
    let stpfiledir = match sftp.readdir(path) {
        Ok(dir) => dir,
        Err(e) => {
            let faild_str = format!("failed: {}", e.message());
            let msg = repr_message(id, code, &faild_str, 100);
            sub_nofity(sender, msg.clone());
            return msg;
        }
    };
    let task_count = stpfiledir.iter().count() as u32;
    let mut task_i = 1;
    let mut msg_body: Vec<DirInfo> = Vec::new();

    let save_dir = format!("{}/{}", &data_dir, &task.msg.root);
    exits_create(&save_dir);

    for (filename, stat) in stpfiledir.iter() {
        // 下载文件到本地
        let file_path = filename.as_path();
        let save_dir = format!(
            "{}/{}",
            &save_dir,
            &file_path.file_name().unwrap().to_str().unwrap()
        );
        let msg = save_file(&sftp, &file_path, &save_dir, stat.is_dir());
        let msg = repr_message(
            id,
            code.clone(),
            &msg,
            calculate_percentage(task_i, task_count),
        );
        // 发布任务进度
        sub_nofity(sender, msg);
        task_i += 1;
        msg_body.push(DirInfo {
            filename: file_path.to_str().unwrap().to_string(),
            size: stat.size.unwrap(),
            is_dir: stat.is_dir(),
        });
    }
    let msg_body = serde_json::to_string(&msg_body).unwrap();

    let result = repr_message(task.id.as_str(), code, msg_body.as_str(), 100);
    result
}

// 错误消息处理
pub fn faild_process(data: Value) -> String {
    let code: MessageCode = MessageCode::InvalidCode;
    let faild_code = data["code"].as_str().unwrap();
    let id = data["id"].as_str().unwrap();
    let result = repr_message(
        id,
        code,
        format!("faild: not find `{}` processing task", faild_code).as_str(),
        100,
    );
    result
}

// 快速创建回复消息
pub fn repr_message(id: &str, code: MessageCode, msg: &str, process: u32) -> String {
    let repr_message = ReprMessage {
        code: MessageCode::ReprMessage,
        id: id.to_string(),
        msg: ReprMessageMsg {
            code,
            process,
            body: msg.to_string(),
        },
    };
    serde_json::to_string(&repr_message).unwrap().to_string()
}

// 退出服务器
pub fn exit_karasync() -> String {
    exit(0)
}
#[test]
fn test_stirng_to_enum() {
    let code = "CloneProjected";
    let json_str = format!(r#""{}""#, code);
    println!("{}", json_str);
    let _code: MessageCode = serde_json::from_str(json_str.as_str()).expect("not pase");
    match _code {
        MessageCode::CloneProjected => {
            println!("Ok")
        }
        _ => {
            println!("Error");
        }
    }
}

#[test]
fn test_repr_message() {
    repr_message("asd", MessageCode::CloneProjected, "test", 109);
}
