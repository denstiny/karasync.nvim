#![allow(dead_code)]
use std::{net::TcpStream, path::Path, rc::Rc};

use logger::{error, warn};
use ssh2::Session;

pub struct ProjectUnify {}

#[derive(Clone)]
pub enum Password {
    Sshkey,
    Password(String),
}

pub struct Auth<'a> {
    pub addr: &'a str,
    pub user: &'a str,
    pub auth: Password,
}

impl ProjectUnify {
    /// 上传项目到服务器
    ///
    /// * `project_path`: 本地项目路径
    /// * `save_path`: 服务器项目路径
    pub fn upload(auth: &Auth, project_path: &Path, save_path: &Path) {}

    /// 下载项目到本地
    ///
    /// * `auth`: 认证结构体
    /// * `path`: 远程的项目路径
    /// * `save_path`: 本地项目路径
    pub fn download(auth: &Auth, path: &Path, save_path: &Path) {}

    fn auth(auth: &Auth) -> Result<Session, Box<dyn std::error::Error>> {
        let tcp = TcpStream::connect(auth.addr)?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake()?;
        match auth.auth.clone() {
            Password::Sshkey => sess.userauth_agent(&auth.user)?,
            Password::Password(passwd) => sess.userauth_password(&auth.user, &passwd)?,
        };

        Ok(sess)
    }
}

#[cfg(test)]
mod test {

    use crate::{Auth, Password, ProjectUnify};

    #[test]
    fn test_auth() {
        ProjectUnify::auth(&Auth {
            addr: "127.0.0.1:22",
            user: "root",
            auth: Password::Sshkey,
        })
        .unwrap();
    }
}
