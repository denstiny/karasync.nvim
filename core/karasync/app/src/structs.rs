use project_unify::Password;
use remotehub::JsonStruct;
use serde::{Deserialize, Serialize};

JsonStruct! {
    UploadProject {
        path: String
    }
}

JsonStruct! {
    DownloadProject {
        server_addr: String,
        server_path: String,
        user: String,
        password: Password,
        local_dir: String,
        msgid: String,
        code: String
    }
}

JsonStruct! {
    ProjectConfg {
        server_addr: String,
        server_path: String,
        user: String,
        login: Password
    }
}

#[cfg(test)]
mod test {
    use super::ProjectConfg;
    use project_unify::Password;

    #[test]
    fn test_project_config() {
        let conf = ProjectConfg {
            server_addr: "127.0.0.1:22".to_string(),
            server_path: "/home/aero".to_string(),
            user: "POa".to_string(),
            login: Password::Password("asd".to_owned()),
        };
        println!("{}", serde_json::to_string_pretty(&conf).unwrap());
    }
}
