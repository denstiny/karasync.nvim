pub mod error;
use error::ErrorCode;
use error::SSHError;
use ssh2::{DisconnectCode, Session};
use std::{io::Read, net::TcpStream};

pub struct SSHManager {
    pub ip: String,
    pub port: u16,
    sshsession: Session,
}

impl SSHManager {
    /// 创建一个新的ssh管理
    ///
    /// * `ip`: &str
    /// * `port`: &u16
    pub fn new(ip: &str, port: u16) -> Self {
        let mut session = Session::new().unwrap();
        let addr = format!("{}:{}", ip, port);
        let tcpsession = TcpStream::connect(addr).unwrap();
        session.set_tcp_stream(tcpsession);
        Self {
            ip: ip.to_owned(),
            port,
            sshsession: session,
        }
    }

    /// 在远程执行命令
    /// * `cmd`: &str
    pub fn exec(&self, cmd: String) -> Result<String, SSHError> {
        let mut channel = self.sshsession.channel_session().unwrap();

        match channel.exec(cmd.as_str()) {
            Ok(_) => (),
            Err(e) => {
                return Err(SSHError::sinp(
                    ErrorCode::Exec,
                    e.message().to_owned().as_str(),
                ));
            }
        };
        // expect("failed: exec cmd error");
        let mut buf = String::new();
        match channel.read_to_string(&mut buf) {
            Ok(t) => {
                if t > 0 {
                    Ok(buf)
                } else {
                    Err(SSHError::sinp(
                        ErrorCode::ReadExec,
                        "read exec output length 0",
                    ))
                }
            }
            Err(_) => Err(SSHError::sinp(ErrorCode::ReadExec, "no read exec output")),
        }
    }

    /// 通过用户名和密码验证登陆
    ///
    /// * `username`: &str
    /// * `password`: &str
    pub fn userauth_password(&mut self, username: String, password: String) -> bool {
        match self.sshsession.handshake() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("handshake failed: {}", e.message());
                return false;
            }
        };

        match self
            .sshsession
            .userauth_password(username.as_str(), password.as_str())
        {
            Ok(_) => (),
            Err(e) => {
                eprintln!("userauth_password: {}", e.message());
                return false;
            }
        }
        true
    }

    pub fn read_file(&mut self) -> String {
        "".to_string()
    }
}

impl Drop for SSHManager {
    fn drop(&mut self) {
        self.sshsession
            .disconnect(Some(DisconnectCode::ByApplication), "", Some("by"))
            .unwrap();
        println!("close SSHManager");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut ssh = SSHManager::new("127.0.0.1", 22);
        ssh.userauth_password("root".to_owned(), "asd".to_owned());
        match ssh.exec("ls -al".to_owned()) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{}", e.msg),
        }
        // utils::cmd::test();
    }
}
