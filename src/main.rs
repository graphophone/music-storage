#[tokio::main]
async fn main() {
    let conf = music_storage::config::Config::build("config/config.local.toml")
        .expect("failed to read config");
    music_storage::run(&conf)
        .await
        .expect("failure during execution");
}
