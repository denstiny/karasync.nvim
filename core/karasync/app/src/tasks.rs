use std::{fs, path::Path};

use logger::{info, warn};
use project_unify::{Auth, ProjectUnify};
use remotehub::{structs::ReprMessage, utils::repr_message, Arc, Client, Mutex};
use serde_json::{json, Value};

use crate::structs::{DownloadProject, ProjectConfg, UploadProject};

pub async fn task_distribute(client: Arc<Mutex<Client>>, msg: Value) {
    info!("---------");
    match serde_json::from_value::<ReprMessage>(msg.to_owned()) {
        Ok(msg) => match msg.code.as_str() {
            "UploadProject" => upload_porject(client.clone(), msg.msg).await,
            _ => {
                let result = repr_message(&msg.msgid, "InvalidCode", "not find code handle");
                let _ = client.lock().await.send(result).await;
            }
        },
        Err(e) => {
            warn!("{} {}", e.to_string(), msg.to_owned());
            let msg = repr_message("nil", "InvalidCode", &e.to_string());
            let _ = client.lock().await.send(msg).await;
        }
    };
}

#[derive(Debug)]
enum TaskErrorKind {
    LoadProject,
}

#[derive(Debug)]
#[allow(dead_code)]
struct TaskError {
    pub kind: TaskErrorKind,
    pub msg: &'static str,
}

impl std::fmt::Display for TaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for TaskError {}

async fn load_project_config(path: &Path) -> Result<ProjectConfg, TaskError> {
    match fs::read_to_string(path) {
        Ok(text) => match serde_json::from_str::<ProjectConfg>(&text) {
            Ok(value) => {
                return Ok(value);
            }
            Err(_) => {
                warn!("failed: load project config");
                Err(TaskError {
                    kind: TaskErrorKind::LoadProject,
                    msg: "load project config",
                })
            }
        },
        Err(_) => {
            warn!("failed: not read project config: {:?}", path);
            Err(TaskError {
                kind: TaskErrorKind::LoadProject,
                msg: "not find project config file",
            })
        }
    }
}

pub async fn download_project(client: Arc<Mutex<Client>>, msg: Value) {
    if let Ok(download) = serde_json::from_value::<DownloadProject>(msg.clone()) {
        match ProjectUnify::download(
            &Auth {
                addr: &download.server_addr,
                user: &download.user,
                auth: download.password,
            },
            Path::new(&download.server_path),
            Path::new(&download.local_dir),
        ) {
            Ok(_) => {
                let _ = client
                    .lock()
                    .await
                    .send(repr_message(
                        &download.msgid,
                        &download.code,
                        &json!({
                        "code": "OK",
                        })
                        .to_string(),
                    ))
                    .await;
            }
            Err(e) => {
                let _ = client
                    .lock()
                    .await
                    .send(repr_message(
                        &download.msgid,
                        &download.code,
                        &json!({
                        "code": "Failed",
                        "msg": e.to_string(),
                        })
                        .to_string(),
                    ))
                    .await;
            }
        };
    }
}

pub async fn upload_porject(client: Arc<Mutex<Client>>, msg: Value) {
    if let Ok(upload) = serde_json::from_value::<UploadProject>(msg.clone()) {
        let config_path = Path::new(&upload.path);
        if let Ok(config) = load_project_config(config_path).await {
            match ProjectUnify::upload(
                &Auth {
                    addr: config.server_addr.as_str(),
                    user: config.user.as_str(),
                    auth: config.login,
                },
                config_path.parent().unwrap(),
                Path::new(&config.server_path),
            ) {
                Ok(_) => {
                    tokio::spawn(async move {
                        let mut client = client.lock().await;
                        let _ = client
                            .send(repr_message(
                                "nil",
                                "UploadProject",
                                &json!({"code": "Ok"}).to_string(),
                            ))
                            .await;
                    });
                }
                Err(e) => {
                    let emsg = e.to_string();
                    tokio::spawn(async move {
                        let mut client = client.lock().await;
                        let _ = client
                            .send(repr_message(
                                "nil",
                                "UploadProject",
                                &json!({"code": "Failed","msg": emsg}).to_string(),
                            ))
                            .await;
                    });
                }
            }
        }
    } else {
        let mut client = client.lock().await;
        let _ = client
            .send(repr_message(
                "InvalidParam",
                "nil",
                "Instruction parameter error",
            ))
            .await;
        warn!("failed: Instruction parameter error {}", msg)
    }
}
