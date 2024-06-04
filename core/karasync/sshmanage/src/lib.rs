pub mod utils;
use ssh2::{Session, Sftp};
use std::{
    io::{Read, Write},
    net::TcpStream,
};
use structs::SSHLoginType;

pub struct SshSession {
    session: ssh2::Session,
}

impl SshSession {
    pub fn create(
        ip: String,
        port: u16,
        user: &str,
        login: &SSHLoginType,
    ) -> Result<Self, ssh2::Error> {
        let mut session = Session::new().unwrap();
        let addr = format!("{}:{}", ip, port);
        let tcpsession = TcpStream::connect(addr).unwrap();
        session.set_tcp_stream(tcpsession);
        session.handshake()?;
        match login {
            SSHLoginType::SSHKEY => session.userauth_agent(user)?,
            SSHLoginType::SSHPASSWORD(password) => session.userauth_password(user, &password)?,
        };
        Ok(Self { session })
    }

    pub fn sftp(&self) -> Result<Sftp, ssh2::Error> {
        match self.session.sftp() {
            Ok(sftp) => Ok(sftp),
            Err(e) => Err(e),
        }
    }

    pub fn cmd(&mut self, cmds: Vec<String>) -> Result<String, ssh2::Error> {
        let mut channel = self.session.channel_session()?;
        channel.shell()?;
        let mut s = String::new();
        for cmd in cmds {
            channel.write_all(cmd.as_bytes()).unwrap();
            channel.write_all(b"\n").unwrap();
        }
        channel.send_eof().unwrap();
        channel.read_to_string(&mut s).unwrap();
        Ok(s)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    #[test]
    fn test_ssh_exec_cmd() {
        let ip = String::from_str("127.0.0.1").unwrap();
        let port: u16 = 22;
        let user = String::from_str("root").unwrap();
        let password = String::from_str("asd").unwrap();
        let mut session =
            SshSession::create(ip, port, &user, SSHLoginType::SSHPASSWORD(password)).unwrap();
        println!(
            "{}",
            session
                .cmd(vec!["cd /root/Public".to_string(), "ls".to_string()])
                .unwrap()
        );
        println!(
            "{}",
            session
                .cmd(vec!["cd /home/denstiny".to_string(), "ls".to_string()])
                .unwrap()
        )
    }
}
