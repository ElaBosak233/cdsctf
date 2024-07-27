pub mod docker;
pub mod k8s;
pub mod traits;

use std::sync::Arc;

use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use tracing::error;
use traits::Container;

use crate::config::get_app_config;

static PROVIDER: Lazy<Mutex<Option<Arc<dyn Container>>>> = Lazy::new(|| Mutex::new(None));

pub async fn init() {
    let provider: Arc<dyn Container> = match get_app_config().container.provider.as_str() {
        "docker" => Arc::new(docker::Docker::new()),
        "k8s" => Arc::new(k8s::K8s::new()),
        _ => {
            error!("Unsupported container provider");
            return;
        }
    };

    {
        let mut global_container = PROVIDER.lock().await;
        *global_container = Some(provider.clone());
    }

    get_container().await.init().await;
}

pub async fn get_container() -> Arc<dyn Container> {
    let global_container = PROVIDER.lock().await;
    return global_container.as_ref().unwrap().clone();
}
