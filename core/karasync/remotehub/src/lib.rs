mod enums;
pub mod structs;
pub mod utils;

pub use std::{collections::HashMap, sync::Arc};

use enums::{RPCError, RPCErrorKind};
use logger::{error, info, warn};
pub use serde_json::Value;
use structs::InitClient;
use tokio::select;
pub use tokio::{
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

    pub async fn send(&mut self, msg: String) -> Result<(), Box<dyn std::error::Error>> {
        Ok(self.sender.send(msg).await?)
    }
}

type CachType = Arc<Mutex<HashMap<String, Arc<Mutex<Client>>>>>;
pub struct Rpc {
    cache: CachType,
    sender: Sender<(Arc<Mutex<Client>>, Value)>,
    receiver: Receiver<(Arc<Mutex<Client>>, Value)>,
}

impl Rpc {
    pub async fn get_cache(&self) -> CachType {
        self.cache.clone()
    }

    pub fn new() -> Self {
        let (tx, rd) = mpsc::channel::<(Arc<Mutex<Client>>, Value)>(10);
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

        let sender = self.sender.clone();
        let cache = self.cache.clone();
        let future = tokio::spawn(async move {
            loop {
                let (socket, _) = listener
                    .accept()
                    .await
                    .expect("Failed to accept connection");

                // 对于每个连接，生成一个新的异步任务来处理消息。
                let sender = sender.clone();
                let cache = cache.clone();
                tokio::spawn(async move {
                    process_connection(socket, cache, sender).await;
                });
            }
        });
        future
    }
    pub async fn recv(&mut self) -> Option<(Arc<Mutex<Client>>, Value)> {
        Some(self.receiver.recv().await?)
    }
}

#[allow(unreachable_patterns)]
async fn process_connection(
    socket: tokio::net::TcpStream,
    cache: CachType,
    sys_sender: Sender<(Arc<Mutex<Client>>, Value)>,
) {
    let (mut rd, mut wd) = io::split(socket);
    let (mut sender, mut receiver) = channel::<String>(10);
    let (shown_send, mut shown_recv) = channel::<i32>(1);

    tokio::spawn(async move {
        loop {
            select! {
                val = receiver.recv() => {
                if let Some(message) = val {
                    info!("repr: {}", message.clone());
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
            _val = shown_recv.recv() => {
                info!("close socket");
                drop(receiver);
                drop(wd);
                return;
            }
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

    let arc_client = client.clone();
    let arc_cache = cache.clone();
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
                arc_cache.lock().await.insert(id, arc_client.clone());
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

    let arc_cache = cache.clone();
    tokio::spawn(async move {
        loop {
            match read_fd_value(&mut rd).await {
                Ok(msg) => {
                    if let Err(e) = sys_sender.send((client.clone(), msg)).await {
                        warn!("{}", e.to_string());
                        break;
                    }
                }
                Err(e) => match e.kind {
                    RPCErrorKind::Disconnect | RPCErrorKind::ReadZero => {
                        break;
                    }
                    RPCErrorKind::Parse => {}
                    _ => {}
                },
            }
        }
        arc_cache.lock().await.remove(&client.lock().await.id);
        let _ = shown_send.send(0).await;
    });
}

async fn read_fd_value(rd: &mut ReadHalf<TcpStream>) -> Result<Value, RPCError> {
    let mut buf = vec![0; 3084];
    match rd.read(&mut buf).await {
        Ok(n) => {
            if n == 0 {
                return Err(RPCError {
                    kind: RPCErrorKind::ReadZero,
                    msg: "filad: Read socket fd is zero",
                });
            }
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
