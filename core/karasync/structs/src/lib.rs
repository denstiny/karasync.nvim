use serde::{Deserialize, Serialize};

macro_rules! JsonStruct {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
      #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $type,)*
        }
    };
}

JsonStruct! {
    AsyncTaskMsg {
        percentage: i32,
        body: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AsyncTask<Message> {
    pub code: MessageCode,
    pub msg: Message,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageCode {
    ConnectedOk,
    InvalidCode,
    PushProjected,
    CloneProjected,
    ReprMessage,
    ExitServer,
}

JsonStruct! {
    Message {
        msg: String,
        code: MessageCode,
    }
}

JsonStruct! {
    AsyncGitClone {
        host: String,
        path: String,
        save_dir: String,
        user: String,
        password: String,
    }
}

// 回复消息的内容
JsonStruct! {
    ReprMessageMsg {
        code: MessageCode, // 回复代码
        process: u32, // 进度
        body: String // 消息文本
    }
}

// 回复消息结构体
JsonStruct! {
    ReprMessage {
        code: MessageCode, // 回复代码状态
        id: String, // id
        msg: ReprMessageMsg //主体
    }
}

// 文件结构体
JsonStruct! {
    DirInfo {
      filename: String,
      size: u64,
      is_dir: bool
    }
}

// 配置文件结构体

#[derive(Debug, Serialize, Deserialize)]
pub enum SSHLoginType {
    SSHKEY(String),
    SSHPASSWORD(String),
}

JsonStruct! {
    Project {
        user: String,
        remote: String,
        login: SSHLoginType
    }
}
