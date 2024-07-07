use crate::structs::ReprMessage;

// 快速创建回复消息
pub fn repr_message(id: &str, code: &str, msg: &str) -> String {
    let repr_message = ReprMessage {
        code: code.to_owned(),
        msgid: id.to_owned(),
        msg: msg.to_owned(),
    };
    serde_json::to_string(&repr_message).unwrap().to_string()
}
