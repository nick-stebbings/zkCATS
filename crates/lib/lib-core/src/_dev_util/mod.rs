use tokio::sync::OnceCell;
use tracing::info;
use crate::ctx::Error;

mod dev_db;

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - init dev_all", "FOR_DEV_ONLY");

        dev_db::init_dev_db().await.unwrap()
   }).await;
}