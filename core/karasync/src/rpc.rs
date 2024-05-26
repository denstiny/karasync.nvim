use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;
use serde_json::Value;
use tokio::io::AsyncWriteExt;
use tokio::io::Error;
use tokio::io::{AsyncReadExt, AsyncWrite};
use tokio::net::TcpListener;
use tokio::task;

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

#[derive(Serialize, Deserialize, Debug)]
enum MessageCode {
    ConnectedOk,
    InvalidCode,
}

#[derive(Serialize, Deserialize)]
/// RPC reply Struct
///
/// * `msg`: String
/// * `code`: MessageCode
struct Message {
    msg: String,
    code: MessageCode,
}

#[macro_export]
macro_rules! FmtMessage {
    ($msg: expr,$code: expr) => {
        {
            let msg = serde_json::to_string(&Self { $msg.to_owned(), $code }).unwrap();
            return msg.as_bytes()
        }
    };
}

#[macro_export]
macro_rules! create_message {
    ($msg:expr, $code:expr) => {
        serde_json::to_string(&Message {
            msg: $msg.to_owned(),
            code: $code,
        })
        .unwrap()
        .as_bytes()
    };
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
                info!("{}", e)
            }
        });
    }
}

async fn process_connection(mut socket: tokio::net::TcpStream) -> Result<(), RPCError> {
    let mut buffer = [0; 1024];

    // 在客户端连接成功后发送初始消息
    let msg = serde_json::to_string(&Message {
        msg: "Welcome to karasync".to_owned(),
        code: MessageCode::ConnectedOk,
    })
    .unwrap();
    let _ = socket.write(msg.as_bytes()).await;

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                return Err(RPCError {
                    kind: RPCErrorKind::Disconnect,
                    msg: "client quit".to_owned(),
                });
            }

            Ok(n) => {
                // 按需处理接收到的数据，或者直接使用它。
                let data = buffer[..n].to_vec();
                let data = match String::from_utf8(data) {
                    Ok(str) => str,
                    Err(e) => {
                        warn!("Invalid UTF-8 sequence: {}", e);
                        continue;
                    }
                };
                // 将读取的数据解析为JSON
                let json_data: Value = match serde_json::from_str(&data) {
                    Ok(data) => data,
                    Err(e) => {
                        error!("failed to parse JSON from received data; err = {:?}", e);
                        let msg = serde_json::to_string(&Message {
                            msg: "failed to parse JSON from received data".to_owned(),
                            code: MessageCode::InvalidCode,
                        })
                        .unwrap();
                        let _ = socket.write(msg.as_bytes()).await;
                        continue;
                    }
                };

                // 处理客户端的请求
                let _ = socket.write_all(handle_task(json_data).as_bytes()).await;
            }
            Err(e) => {
                error!("failed to read from socket; err = {:?}", e);
                return Err(RPCError {
                    kind: RPCErrorKind::Disconnect,
                    msg: "failed to read from socket".to_owned(),
                });
            }
        };
    }
}

fn handle_task(data: Value) -> String {
    // 代表耗时操作，例如密集计算或同步IO等。
    info!("处理了数据：{:?}", data);
    data.to_string()
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        await_accept("127.0.0.1");
    }
}
