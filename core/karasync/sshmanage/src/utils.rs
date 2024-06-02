use ssh2::Sftp;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

#[allow(dead_code)]
pub fn save_file(sftp: &Sftp, path: &Path, to_path: &str, is_dir: bool) -> String {
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
            Err(err) => return format!("faild: create file {} => {}", to_path, err.to_string()),
        };
        while let Ok(n) = file.read(&mut buf) {
            if n == 0 {
                break;
            }
            match local_file.write_all(&buf[..n]) {
                Ok(_) => (),
                Err(_) => return format!("faild: write file {}", to_path),
            }
        }
    } else {
        exits_create(to_path);
    }
    format!(
        "sucessfully: downloaded file {}",
        path.file_name().unwrap().to_str().unwrap()
    )
}

pub fn exits_create(path: &str) {
    let path = Path::new(path);
    if !path.exists() {
        fs::create_dir_all(path).unwrap()
    }
}
