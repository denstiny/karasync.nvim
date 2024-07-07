#![allow(dead_code)]

mod enums;
mod structs;
mod utils;

use std::{collections::HashMap, sync::Arc};

use enums::{RPCError, RPCErrorKind};
use logger::{error, info, warn};
use serde_json::Value;
use structs::InitClient;
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, ReadHalf},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, channel, Receiver, Sender},
        Mutex,
    },
    task::JoinHandle,
    time,
};

#[derive(Debug, Clone)]
pub struct Client {
    id: String,
    path: String,
    sender: Sender<String>,
}

impl Client {
    pub fn from_value(&mut self, conf: InitClient) {
        self.id = conf.id;
        self.path = conf.path;
    }
}

type CachType = Arc<Mutex<HashMap<String, Arc<Mutex<Client>>>>>;
pub struct Rpc {
    cache: CachType,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

impl Rpc {
    pub fn new() -> Self {
        let (tx, rd) = mpsc::channel::<String>(10);
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            sender: tx,
            receiver: rd,
        }
    }
    pub async fn accept(&mut self, addr: &str) -> JoinHandle<()> {
        let listener = match TcpListener::bind(addr).await {
            Ok(t) => {
                info!("start server: {}", addr);
                t
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        };

        let cache = self.cache.clone();
        let sender = self.sender.clone();
        let future = tokio::spawn(async move {
            loop {
                let (socket, _) = listener
                    .accept()
                    .await
                    .expect("Failed to accept connection");

                let inal_cache = cache.clone();
                // 对于每个连接，生成一个新的异步任务来处理消息。
                let sender = sender.clone();
                tokio::spawn(async move {
                    process_connection(socket, inal_cache, sender).await;
                });
            }
        });
        future
    }
    async fn recv(&mut self) -> Option<String> {
        Some(self.receiver.recv().await?)
    }
    async fn send(&mut self, value: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.sender.send(value).await?)
    }
}

async fn process_connection(
    socket: tokio::net::TcpStream,
    cache: CachType,
    sys_sender: Sender<String>,
) {
    let (mut rd, mut wd) = io::split(socket);
    let (mut sender, mut receiver) = channel::<String>(10);

    tokio::spawn(async move {
        loop {
            if let Some(message) = receiver.recv().await {
                info!("receiver: {}", message.clone());
                let msg = format!("{}\n", message);
                if let Err(e) = wd.write_all(msg.as_bytes()).await {
                    warn!("faild: {}", e.to_string());
                    break;
                }
            } else {
                info!("close socket");
                return;
            }
        }
    });

    if let Err(_) = ok_message(&mut sender).await {
        error!("faild: send ok message");
        return;
    }

    let client = Arc::new(Mutex::new(Client {
        id: "".to_owned(),
        path: "".to_owned(),
        sender: sender.clone(),
    }));

    let cache = cache.clone();
    let arc_client = client.clone();
    let mut rd = match time::timeout(time::Duration::from_secs(3), async {
        match read_fd_value(&mut rd).await {
            Ok(msg) => {
                let client_conf: InitClient = match serde_json::from_value(msg) {
                    Ok(data) => data,
                    Err(_) => {
                        info!("drop socket");
                        drop(rd);
                        return Err(RPCError {
                            kind: RPCErrorKind::Parse,
                            msg: "parse init client",
                        });
                    }
                };
                let id;
                {
                    let mut client = arc_client.lock().await;
                    client.from_value(client_conf);
                    id = client.id.to_owned();
                }
                cache.lock().await.insert(id, arc_client);
            }
            Err(e) => {
                info!("drop socket");
                drop(rd);
                return Err(e);
            }
        };
        return Ok(rd);
    })
    .await
    {
        Ok(_rd) => match _rd {
            Ok(_rd) => {
                info!("client login successful");
                _rd
            }
            Err(e) => {
                warn!("{}", e.to_string());
                return;
            }
        },
        Err(_) => {
            info!("client authentication timeout");
            return;
        }
    };

    let cache = cache.clone();
    tokio::spawn(async move {
        loop {
            match read_fd_value(&mut rd).await {
                Ok(msg) => {
                    if let Err(e) = sys_sender.send(msg.to_string()).await {
                        warn!("{}", e.to_string());
                        break;
                    }
                }
                Err(e) => match e.kind {
                    RPCErrorKind::Disconnect | RPCErrorKind::ReadZero => {
                        drop(rd);
                        cache.lock().await.remove(&client.lock().await.id);
                        break;
                    }
                    RPCErrorKind::Parse => {}
                    _ => {}
                },
            }
        }
    });
}

async fn read_fd_value(rd: &mut ReadHalf<TcpStream>) -> Result<Value, RPCError> {
    let mut buf = vec![0; 3084];
    match rd.read(&mut buf).await {
        Ok(0) => Err(RPCError {
            kind: RPCErrorKind::ReadZero,
            msg: "filad: Read socket fd is zero",
        }),
        Ok(n) => {
            let data = match String::from_utf8(buf) {
                Ok(mut str) => {
                    info!("received: {}", str);
                    str.truncate(n);
                    str
                }
                Err(e) => {
                    warn!("Invalid UTF-8 sequence: {}", e.to_string());
                    return Err(RPCError {
                        kind: RPCErrorKind::ReadZero,
                        msg: "faild: socket buffer is not parse",
                    });
                }
            };

            // 解析用户发送的数据
            let json_data: Value = match serde_json::from_str(&data) {
                Ok(data) => data,
                Err(_e) => {
                    error!("failed to parse JSON from received data; err = {:?}", _e);
                    return Err(RPCError {
                        kind: RPCErrorKind::Parse,
                        msg: "message parse json",
                    });
                }
            };
            return Ok(json_data);
        }
        Err(e) => {
            error!("{}", e.to_string());
            Err(RPCError {
                kind: RPCErrorKind::Disconnect,
                msg: "connection is Disconnect",
            })
        }
    }
}

async fn ok_message(wd: &Sender<String>) -> Result<(), Box<dyn std::error::Error + 'static>> {
    let msg = utils::repr_message("nil", "ConnectedOk", "Welcome to Karasync");
    Ok(wd.send(msg).await?)
}
