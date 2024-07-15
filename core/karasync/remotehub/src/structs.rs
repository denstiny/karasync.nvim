use serde::{Deserialize, Serialize};
use serde_json::Value;
#[macro_export]
macro_rules! JsonStruct {
    ($name:ident { $($field:ident: $type:ty),* $(,)? }) => {
      #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $type,)*
        }
    };
}

// 回复消息结构体
JsonStruct! {
    ReprMessage {
        code: String, // 回复代码状态
        msgid: String, // id
        msg: Value //主体
    }
}

// 回复消息结构体
JsonStruct! {
    InitClient {
        id: String,
        path: String,
    }
}
