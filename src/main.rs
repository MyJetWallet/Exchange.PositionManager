use std::{sync::Arc, time::Duration};

use position_manager::{setup_sb, AppContext, Settings};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let settings = Settings::from_file("./settings.json".into()).await;
    let app = Arc::new(AppContext::new());

    let sb_client = setup_sb(
        &settings.sb_host_port,
        &"position-manager".into(),
        app.clone(),
    )
    .await;

    sb_client.start().await;

    app.set_sb_client(sb_client).await;

    loop {
        sleep(Duration::from_secs(5)).await;
    }
}
