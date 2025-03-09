use crate::{ctx::Error, model::ModelManager};
use tokio::sync::OnceCell;
use tracing::info;

mod dev_db;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init dev_all", "FOR_DEV_ONLY");

        dev_db::init_dev_db().await.unwrap()
    })
    .await;
}

// Initialize for tests
pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT
        .get_or_init(|| async {
            info!("{:<12} - init dev_all", "FOR_TEST_ONLY");

            init_dev().await;
            ModelManager::new().await.unwrap()
        })
        .await;

    mm.clone()
}
