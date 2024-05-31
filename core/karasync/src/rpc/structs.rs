use serde::{Deserialize, Serialize};
macro_rules! TaskStruct {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
      #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $type,)*
        }
    };
}

TaskStruct! {
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
    AsyncProjected,
    CloneProjected,
    ReprMessage,
}

TaskStruct! {
    Message {
        msg: String,
        code: MessageCode,
    }
}

TaskStruct! {
    AsyncGitClone {
        host: String,
        path: String,
        root: String,
        user: String,
        password: String,
    }
}

// 回复消息的内容
TaskStruct! {
    ReprMessageMsg {
        code: MessageCode, // 回复代码
        process: u32, // 进度
        body: String // 消息文本
    }
}

// 回复消息结构体
TaskStruct! {
    ReprMessage {
        code: MessageCode, // 回复代码状态
        id: String, // id
        msg: ReprMessageMsg //主体
    }
}

TaskStruct! {
    DirInfo {
      filename: String,
      size: u64,
      is_dir: bool
    }
}
