use std::{time::Duration, sync::Arc};

use my_service_bus_tcp_client::MyServiceBusClient;
use position_manager::{setup_sb_subscribe, AppContext, Settings};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let settings = Settings::from_file("./settings.json".into()).await;
    let mut sb_client = MyServiceBusClient::new(
        &settings.sb_host_port,
        &settings.app_name,
        "1.0.0",
        Duration::new(3, 0),
        Duration::new(3, 0),
    );

    let app = Arc::new(AppContext::new());
    setup_sb_subscribe(app.clone(), &sb_client).await;

    sb_client.start().await;

    app.set_sb_client(sb_client).await;

    loop {
        sleep(Duration::from_secs(5)).await;
    }
}
