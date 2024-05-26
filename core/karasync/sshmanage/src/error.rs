use core::fmt;
use std::borrow::Cow;

#[derive(Debug)]
pub enum ErrorCode {
    Exec,
    ReadExec,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct SSHError {
    pub code: ErrorCode,
    pub msg: Cow<'static, str>,
}

impl fmt::Display for SSHError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed: {}", self.msg)
    }
}
impl SSHError {
    pub fn sinp(code: ErrorCode, msg: &str) -> Self {
        Self {
            code,
            msg: Cow::from(msg.to_owned()),
        }
    }
}

impl std::error::Error for SSHError {}
