mod dev_db;

use tokio::sync::OnceCell;
use tracing::{debug, info};

pub async fn init_dev_db()  {
    static INIT: OnceCell<()> = OnceCell::const_new();

    INIT.get_or_init(|| async {
        info!("{:<12} - _dev_utils::init_dev_db()", "FOR-DEV-ONLY");

        dev_db::init_dev_db().await.unwrap();
    }).await;
}