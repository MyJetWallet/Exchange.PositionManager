use std::sync::Arc;

use my_service_bus_tcp_client::MyServiceBusClient;

use crate::app::AppContext;

use super::subscribers::{bid_ask_subscribe, open_position_subscribe};

pub async fn setup_sb_subscribe(
    app: Arc<AppContext>,
    client: &MyServiceBusClient,
) {
    bid_ask_subscribe(app.clone(), client).await;
    open_position_subscribe(app.clone(), client).await;
}
