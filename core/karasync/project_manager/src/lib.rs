use sshmanage::utils;
use sshmanage::SshSession;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::{net::SocketAddr, path::Path};
use structs::AsyncGitClone;
use structs::Project;

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
pub fn update_project_conf(project: Project, data_file: &Path) -> Result<(), String> {
    let mut f = try_or_return_err!(File::create(data_file));
    let buf = try_or_return_err!(serde_json::to_string_pretty(&project));
    match f.write_all(buf.as_bytes()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// 加载项目文件，返回一个Project结构
///
/// * `data_file`: 存储文件路径
pub fn load_project_conf(data_file: &Path) -> Result<Project, String> {
    let mut f = try_or_return_err!(File::open(data_file));
    let mut buf = String::new();
    try_or_return_err!(f.read_to_string(&mut buf));
    Ok(try_or_return_err!(serde_json::from_str::<Project>(&buf)))
}

/// 克隆远程项目
///
/// * `conf`: 克隆远程项目
/// * `notify`: 进度通知回调
pub fn project_dir_clone(
    conf: &AsyncGitClone,
    notify: &dyn Fn(String, u32, u32),
) -> Result<(), String> {
    // 检查host是否正确
    let socket_addr: SocketAddr = match conf.host.parse() {
        Ok(addr) => addr,
        Err(e) => return Err(e.to_string()),
    };
    let (ip, port) = (socket_addr.ip().to_string(), socket_addr.port());
    let user = conf.user.as_str();
    let login = conf.login.clone();

    let ssh = match SshSession::create(ip, port, user, &login) {
        Ok(session) => session,
        Err(e) => return Err(e.to_string()),
    };

    let sftp = try_or_return_err!(ssh.sftp());
    let path = Path::new(&conf.path);
    let save_dir = Path::new(&conf.save_dir).join(path.file_name().unwrap());
    // 检查存储路径是否存在，不存在则创建
    utils::exits_create(&save_dir);

    let mut cursor = 0;
    //let mut msg_body: Vec<DirInfo> = Vec::new();
    let files = try_or_return_err!(sftp.readdir(path));
    let files_count = files.iter().count() as u32;
    for (file, stat) in files.iter() {
        let file_path = file.as_path();
        let save_dir = save_dir.join(file_path.file_name().unwrap());
        let msg = utils::clone_files(&sftp, file_path, save_dir.as_path(), stat.is_dir());

        // 回调函数发送进度
        cursor += 1;
        notify(msg, cursor, files_count);
        //msg_body.push(DirInfo {
        //    filename: file_path.to_str().unwrap().to_string(),
        //    size: stat.size.unwrap(),
        //    is_dir: stat.is_dir(),
        //});
    }

    // 更新当前项目的配置文件
    match update_project_conf(
        Project {
            user: conf.user.clone(),
            remote: conf.host.clone(),
            login: login.clone(),
            path: conf.save_dir.clone(),
            from: conf.path.clone(),
        },
        save_dir.join(".project.json").as_path(),
    ) {
        Ok(_) => {
            //let msg_body = serde_json::to_string(&msg_body).unwrap();
            //notify(msg_body, 100, 100);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// 将当前修改上传到远程
///
/// * `conf`:
/// * `notify`:
pub fn project_dir_push(conf: &Project, notify: &dyn Fn(String, u32, u32)) -> Result<(), String> {
    let (ip, port) = match conf.remote.parse::<SocketAddr>() {
        Ok(socket_addr) => (socket_addr.ip().to_string(), socket_addr.port()),
        Err(e) => return Err(e.to_string()),
    };
    let login = conf.login.clone();
    let ssh = try_or_return_err!(SshSession::create(ip, port, &conf.user, &login));
    let sftp = try_or_return_err!(ssh.sftp());
    let files_iter = try_or_return_err!(fs::read_dir(conf.path.clone()));
    let files_count = try_or_return_err!(fs::read_dir(conf.path.clone())).count() as u32;

    let mut files_index = 0;
    let to_path = Path::new(conf.path.as_str());
    for file in files_iter {
        match file {
            Ok(file) => {
                let path = file.path();
                let msg = utils::push_files(&sftp, &path, to_path, path.is_dir());
                files_index += 1;
                notify(msg, files_index, files_count);
            }
            Err(e) => return Err(e.to_string()),
        }
    }

    notify(String::new(), 100, 100);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let path = Path::new("/home/denstiny/.local/share/nvim/karasync/Public/project.json");
        match load_project_conf(path) {
            Ok(project) => {
                let jsn_project = serde_json::to_string(&project).unwrap();
                println!("成功: {}", jsn_project);
            }
            Err(e) => {
                println!("错误: {}", e);
            }
        }
    }
}
