use lazy_static::lazy_static;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_json::{json, Result as JsonResult};
use std::collections::{HashMap, VecDeque};
use std::net::TcpStream;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use structs::{Message, MessageCode};
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::io::Error;
use tokio::io::{AsyncReadExt, AsyncWrite};
use tokio::net::TcpListener;
use tokio::sync::{Mutex, Notify};
use tokio::task;

mod fast;
mod processing;
use self::processing::repr_message;

//type ArcQueue = Arc<(Mutex<VecDeque<String>>, Notify)>;

#[derive(Debug)]
enum RPCErrorKind {
    Disconnect,
}

#[derive(Debug)]
#[allow(dead_code)]
struct RPCError {
    kind: RPCErrorKind,
    msg: String,
}

impl std::fmt::Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for RPCError {}

#[tokio::main]
pub async fn await_accept(addr: &str) {
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to port");
    info!("bind tcp server port");

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        // 对于每个连接，生成一个新的异步任务来处理消息。
        tokio::spawn(async move {
            let result = process_connection(socket).await;
            if let Err(e) = result {
                error!("{}", e)
            }
        });
    }
}

async fn process_connection(socket: tokio::net::TcpStream) -> Result<(), RPCError> {
    // 将socket 分离读写分离
    let (mut rd, mut wd) = io::split(socket);

    let (sender, receiver) = mpsc::channel::<String>();
    //let list: ArcQueue = Arc::new((Mutex::new(VecDeque::new()), Notify::new()));
    //let arc_sender = Arc::new(sender);
    // 等读取线程处理完毕唤醒写入线程写入数据
    tokio::spawn(async move {
        loop {
            match receiver.recv() {
                Ok(message) => {
                    info!("receiver: {}", message.clone());
                    let msg = format!("{}\n", message);
                    wd.write_all(msg.as_bytes()).await.unwrap();
                }
                Err(e) => {
                    warn!("faild: {}", e.to_owned());
                    break;
                }
            }
        }
    });

    // begin: 在客户端连接成功后发送初始消息
    let msg = repr_message("nil", MessageCode::ConnectedOk, "Welcome to Karasync", 100);
    sender.send(msg).unwrap();
    // end

    // 等待接收数据
    // 处理客户端发送的数据
    //let _sender = Arc::clone(&arc_sender);
    let _sender = sender.clone();
    let hand: Result<(), RPCError> = tokio::spawn(async move {
        loop {
            let mut buf = vec![0; 3084];
            match rd.read(&mut buf).await {
                Ok(0) => {
                    return Err(RPCError {
                        kind: RPCErrorKind::Disconnect,
                        msg: "client quit".to_owned(),
                    });
                }
                Ok(_n) => {
                    let data = match String::from_utf8(buf) {
                        Ok(mut str) => {
                            info!("received: {}", str);
                            str.truncate(_n);
                            str
                        }
                        Err(e) => {
                            warn!("Invalid UTF-8 sequence: {}", e);
                            continue;
                        }
                    };

                    // 解析用户发送的数据
                    let json_data: Value = match serde_json::from_str(&data) {
                        Ok(data) => data,
                        Err(_e) => {
                            error!("failed to parse JSON from received data; err = {:?}", _e);
                            let msg = serde_json::to_string(&Message {
                                msg: format!(
                                    "failed to parse JSON from received data, length = {}",
                                    _n
                                ),
                                code: MessageCode::InvalidCode,
                            })
                            .unwrap();
                            _sender.send(msg).unwrap();
                            continue;
                        }
                    };

                    // 启动任务分配器
                    let c_sender = _sender.clone();
                    thread::spawn(move || {
                        assign_task(json_data, c_sender);
                    });
                }
                Err(e) => {
                    error!("failed to read from socket; err = {:?}", e);
                    return Err(RPCError {
                        kind: RPCErrorKind::Disconnect,
                        msg: "failed to read from socket".to_owned(),
                    });
                }
            }
        }
    })
    .await
    .unwrap();
    drop(sender);
    match hand {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn assign_task(data: Value, sender: mpsc::Sender<String>) {
    let code = data["code"].as_str().unwrap();
    let json_str = format!(r#""{}""#, code);

    let code: MessageCode = match serde_json::from_str(json_str.as_str()) {
        Ok(c) => c,
        Err(_) => MessageCode::InvalidCode,
    };

    let result = match code {
        MessageCode::CloneProjected => processing::async_project_clone(data, &sender),
        MessageCode::ExitServer => processing::exit_karasync(),
        _ => processing::faild_process(data),
    };
    sender.send(result).unwrap();
    drop(sender);
}

mod tests {
    use super::*;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn it_works() {
        //await_accept("127.0.0.1");
        let str = r#"{"msg":{"comment":"test","branch":"main","user":"denstiny"},"code":"AsyncProjected"}"#;
        let d: Value = serde_json::from_str(str).unwrap();
        println!("{}", d.to_string());
    }
}
