use sshmanage::utils;
use sshmanage::SshSession;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::{net::SocketAddr, path::Path};
use structs::Project;
use structs::{AsyncGitClone, DirInfo};

macro_rules! try_or_return_err {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.to_string()),
        }
    };
}

/// 更新当前项目的配置文件
///
/// * `project`: 项目配置文件
/// * `data_file`: 项目配置文件存储路径
pub fn update_project_conf(project: Project, data_file: &str) -> Result<(), String> {
    let mut f = try_or_return_err!(File::open(data_file));
    let buf = try_or_return_err!(serde_json::to_string(&project));
    match f.write_all(buf.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// 加载项目文件，返回一个Project结构
///
/// * `data_file`: 存储文件路径
pub fn load_project_conf(data_file: &str) -> Result<Project, String> {
    let mut f = try_or_return_err!(File::open(data_file));
    let mut buf = String::new();
    try_or_return_err!(f.read_to_string(&mut buf));
    try_or_return_err!(serde_json::from_str(&buf))
}

/// 克隆远程项目
///
/// * `conf`: 克隆远程项目
/// * `data_dir`: 保存路径
/// * `notify`: 进度通知回调
pub fn project_dir_clone(
    conf: &AsyncGitClone,
    data_dir: &str,
    notify: &dyn Fn(String, u32, u32),
) -> Result<(), String> {
    // 检查host是否正确
    let socket_addr: SocketAddr = match conf.host.parse() {
        Ok(addr) => addr,
        Err(e) => return Err(e.to_string()),
    };
    let (ip, port) = (socket_addr.ip().to_string(), socket_addr.port());
    let user = conf.user.as_str();
    let password = conf.password.as_str();
    let ssh = match SshSession::create(ip, port, user, password) {
        Ok(session) => session,
        Err(e) => return Err(e.to_string()),
    };

    let sftp = try_or_return_err!(ssh.sftp());
    let path = Path::new(&conf.path);
    let files = try_or_return_err!(sftp.readdir(path));
    let files_count = files.iter().count() as u32;
    let mut cursor = 0;
    let mut msg_body: Vec<DirInfo> = Vec::new();
    let save_dir = format!("{}/{}", data_dir, &conf.root);
    // 检查存储路径是否存在，不存在则创建
    utils::exits_create(&save_dir);

    for (file, stat) in files.iter() {
        let file_path = file.as_path();
        let save_dir = format!(
            "{}/{}",
            &save_dir,
            &file_path.file_name().unwrap().to_str().unwrap()
        );
        let msg = utils::save_file(&sftp, &file_path, &save_dir, stat.is_dir());
        cursor += 1;
        // 回调函数发送进度
        notify(msg, cursor, files_count);
        msg_body.push(DirInfo {
            filename: file_path.to_str().unwrap().to_string(),
            size: stat.size.unwrap(),
            is_dir: stat.is_dir(),
        });
    }
    let msg_body = serde_json::to_string(&msg_body).unwrap();
    notify(msg_body, 100, 100);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
