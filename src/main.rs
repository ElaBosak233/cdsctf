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

const BANNER: &str = r#"
      _                 _         _       _
  ___/ | ___  _   _  __| |___  __| | __ _/ | ___
 / __| |/ _ \| | | |/ _` / __|/ _` |/ _` | |/ _ \
| (__| | (_) | |_| | (_| \__ \ (_| | (_| | |  __/
 \___|_|\___/ \__,_|\__,_|___/\__,_|\__,_|_|\___|
                                    Version {{.Version}}
Commit: {{.Commit}}
GitHub: https://github.com/elabosak233/cloudsdale
"#;

#[tokio::main]
async fn main() {
    println!(
        "{}",
        BANNER.replace("{{.Version}}", env!("CARGO_PKG_VERSION"))
    );

    server::bootstrap().await;
}
