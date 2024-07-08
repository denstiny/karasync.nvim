use std::path::Path;

use project_unify::{Auth, Password, ProjectUnify};

fn main() {
    let auth = Auth {
        addr: "127.0.0.1:22",
        user: "***",
        auth: Password::Password(String::from("***")),
    };
    ProjectUnify::download(&auth, Path::new("***"), Path::new("***")).unwrap()
}
