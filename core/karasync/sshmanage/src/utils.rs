use ssh2::Sftp;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

#[allow(dead_code)]
/// 从远端克隆文件到本地
///
/// * `sftp`: ssh2-sftp
/// * `path`: 远端路径
/// * `to_path`: 本地存储路径
/// * `is_dir`:  是否是文件夹
pub fn clone_files(sftp: &Sftp, path: &Path, to_path: &Path, is_dir: bool) -> String {
    //info!(
    //    "from: {}  to: {}  is_dir: {}",
    //    path.to_str().unwrap(),
    //    to_path,
    //    is_dir
    //);
    //let filename = path.file_name().unwrap().to_str().unwrap();
    let path = Path::new(path);
    if !is_dir {
        // TODO: 检查为什么读取失败
        let mut file = match sftp.open(path) {
            Ok(f) => f,
            Err(e) => return format!("faild: {}", e.message()),
        };
        let mut buf = vec![0; 1024];
        let mut local_file = match File::create(to_path) {
            Ok(f) => f,
            Err(err) => {
                return format!(
                    "faild: create file {} => {}",
                    to_path.to_str().unwrap(),
                    err.to_string()
                )
            }
        };
        while let Ok(n) = file.read(&mut buf) {
            if n == 0 {
                break;
            }
            match local_file.write_all(&buf[..n]) {
                Ok(_) => (),
                Err(_) => return format!("faild: write file {}", to_path.display()),
            }
        }
    } else {
        exits_create(&to_path);
        let files = sftp.readdir(path).unwrap();
        for (file, stat) in files.iter() {
            let load_path = file.as_path();
            let save_path = Path::new(to_path).join(file.file_name().unwrap());
            clone_files(sftp, load_path, &save_path, stat.is_dir());
        }
    }
    format!(
        "sucessfully: downloaded file {}",
        path.file_name().unwrap().to_str().unwrap()
    )
}

/// 将本地的项目上传到远程,触发错误时候直接返回错误消息字符串
/// 过滤本地没有修改的文件(如果远端存在这个文件的话)
/// * `sftp`: ssh2-sftp
/// * `path`: 本地项目路径
/// * `to_path`: 远程项目路径
/// * `is_dir`: 是否是文件夹
pub fn push_files(sftp: &Sftp, path: &Path, to_path: &Path, is_dir: bool) -> String {
    String::new()
}

pub fn exits_create(path: &Path) {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path).unwrap()
    }
}
