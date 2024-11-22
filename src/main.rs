use typestate_pattern_usecase::server;

#[tokio::main]
async fn main() {
    server::run_server().await;
}
