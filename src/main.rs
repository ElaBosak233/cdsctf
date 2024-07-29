mod assets;
mod captcha;
mod config;
mod container;
mod database;
mod email;
mod logger;
mod media;
mod model;
mod proxy;
mod repository;
mod server;
mod traits;
mod util;

#[tokio::main]
async fn main() {
    let banner = assets::Assets::get("banner.txt").unwrap();
    println!(
        "{}",
        std::str::from_utf8(banner.data.as_ref())
            .unwrap()
            .replace("{{version}}", env!("CARGO_PKG_VERSION"))
            .replace("{{commit}}", env!("GIT_COMMIT_ID"))
            .replace("{{build_at}}", env!("BUILD_AT"))
    );

    server::bootstrap().await;
}
