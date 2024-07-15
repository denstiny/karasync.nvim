use std::path::Path;

use project_unify::{Auth, Password, ProjectUnify};

fn main() {
    let auth = Auth {
        addr: "127.0.0.1:22",
        user: "root",
        auth: Password::Password(String::from("***")),
    };
    ProjectUnify::upload(
        &auth,
        Path::new("/home/denstiny/Workspace/denstiny/"),
        Path::new("/root/Workspace"),
    )
    .unwrap()
}
