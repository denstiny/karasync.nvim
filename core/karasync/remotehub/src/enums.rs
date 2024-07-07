#[derive(Debug)]
pub enum RPCErrorKind {
    Disconnect,
    ReadZero,
    Parse,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct RPCError {
    pub kind: RPCErrorKind,
    pub msg: &'static str,
}

impl std::fmt::Display for RPCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for RPCError {}
