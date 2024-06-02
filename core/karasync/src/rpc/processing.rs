use crate::config::get_config;
use crate::logger::HandleResult;
use crate::utils::calculate_percentage;
use lazy_static::lazy_static;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use ssh2::{FileStat, Session};
use std::collections::{HashMap, VecDeque};
use std::fmt::format;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::Mutex as stdMutex;
use std::sync::{mpsc, Arc};
use std::{io::Write, net::TcpStream};
use structs::{AsyncGitClone, AsyncTask, MessageCode, Project, ReprMessage, ReprMessageMsg};
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
    let id = task.id.as_str();
    let data_dir = get_config().data_dir;
    let conf = task.msg;
    let project_name = Path::new(conf.path.as_str())
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let callback = |msg: String, index: u32, count: u32| {
        sub_nofity(
            sender,
            repr_message(
                id,
                task.code.clone(),
                msg.as_str(),
                calculate_percentage(index, count),
            ),
        );
    };

    match project_manager::project_dir_clone(&conf, &data_dir, &callback) {
        Ok(msg) => msg,
        Err(e) => sub_nofity(sender, repr_message(id, task.code.clone(), e.as_str(), 100)),
    };

    let project = Project {
        user: conf.user.clone(),
        remote: conf.host,
    };

    let data_file = format!("{}/{}/{}", data_dir, &conf.root, project_name);
    if let Err(e) = project_manager::update_project_conf(project, &data_file) {
        return repr_message(
            id,
            task.code,
            &format!("faild: update project conf {}", e.as_str()),
            100,
        );
    }
    repr_message(
        id,
        task.code.clone(),
        format!("Sucessfully: clone project {}", project_name).as_str(),
        100,
    )
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
