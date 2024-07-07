use logger::init_logger;
use remotehub::Rpc;
#[tokio::main]
async fn main() {
    init_logger("./karsync.log");
    let mut server = Rpc::new();
    let s = server.accept("127.0.0.1:7800").await;
    s.await.unwrap();
}
