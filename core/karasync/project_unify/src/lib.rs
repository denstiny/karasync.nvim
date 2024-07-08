#![allow(dead_code)]
use std::fs::{self, File, OpenOptions};
use std::io::copy;
use std::path::PathBuf;
use std::ptr::slice_from_raw_parts_mut;
use std::thread::sleep;
use std::time::Duration;
use std::{net::TcpStream, path::Path};

use logger::{error, info, warn};
use ssh2::{Session, Sftp};

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
    pub fn upload(
        auth: &Auth,
        local_dir: &Path,
        remote_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sess = Self::auth(auth).map_err(|e| {
            error!("faild: {}", e);
            e
        })?;

        let sftp = sess.sftp()?;
        Self::upload_dir(&sftp, local_dir, remote_path)
    }

    pub fn upload_dir(
        sftp: &Sftp,
        local_path: &Path,
        remote_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = sftp.mkdir(remote_path, 777);
        let entry = fs::read_dir(local_path)?;
        for file in entry {
            let file = file?;
            let local_path = file.path();
            let remote_path = remote_path.join(file.file_name());
            if local_path.is_dir() {
                Self::upload_dir(sftp, &local_path, &remote_path)?;
            } else {
                let mut remote_file = sftp.create(&remote_path)?;
                let mut local_file = File::open(&local_path)?;
                copy(&mut local_file, &mut remote_file)?;
            }
        }
        Ok(())
    }

    /// 下载项目到本地
    pub fn download(
        auth: &Auth,
        remote_path: &Path,
        local_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let sess = match Self::auth(auth) {
            Ok(sess) => sess,
            Err(e) => {
                error!("faild: Authentication failure {:?}", e);
                return Err(e);
            }
        };
        let sftp = sess.sftp()?;
        Self::download_dir(&sftp, remote_path, local_dir)
    }

    fn download_dir(
        sftp: &Sftp,
        remote_path: &Path,
        local_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (file, stat) in sftp.readdir(remote_path)?.iter() {
            if let Some(file_name) = file.file_name() {
                let remote_path = remote_path.join(file_name);
                let local_path = local_dir.join(file_name);
                if stat.is_dir() {
                    exits_create(&local_path);
                    Self::download_dir(sftp, &remote_path, &local_path)?;
                } else {
                    println!("{:?}\n{:?}", remote_path, local_path);
                    let mut remote_file = sftp.open(&remote_path)?;
                    let mut local_file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(local_path)?;
                    copy(&mut remote_file, &mut local_file)?;
                }
            }
        }
        Ok(())
    }

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

pub fn exits_create(path: &Path) {
    let path = Path::new(path);
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap()
    }
}

#[cfg(test)]
mod test {

    use std::{path::Path, str::FromStr};

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

    #[test]
    fn test_upload() {
        let s = Auth {
            addr: "127.0.0.1:22",
            user: "root",
            auth: Password::Password(String::from("***")),
        };
        ProjectUnify::upload(&s, Path::new("**"), Path::new("**")).unwrap();
    }
}
