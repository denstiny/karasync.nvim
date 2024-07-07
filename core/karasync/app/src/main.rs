use std::env;

use logger::{info, init_logger};
use remotehub::Rpc;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprint!("start karsync missing parameter");
        return;
    }
    let data_dir = args[1].to_owned();
    let ip = args[2].to_owned();
    let port = args[3].to_owned();

    init_logger(&format!("{}/karsync.log", data_dir));

    let mut server = Rpc::new();

    let result = server.accept(&format!("{}:{}", ip, port)).await;
    loop {
        match server.recv().await {
            Some((_client, msg)) => {
                info!("recv: {}", msg);
            }
            None => break,
        }
    }
    result.await.unwrap();
}
