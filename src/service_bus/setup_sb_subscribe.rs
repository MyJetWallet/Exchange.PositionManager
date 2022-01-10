use std::sync::Arc;

use my_service_bus_shared::queue::TopicQueueType;
use my_service_bus_tcp_client::MyServiceBusClient;

use crate::app::AppContext;

use super::subscribers::{BidAskSubscriber, OpenPositionSubscriber};

pub async fn setup_sb(sb_host: &String, app_name: &String, app: Arc<AppContext>) -> MyServiceBusClient {
    let sb_client = MyServiceBusClient::new(sb_host, app_name);

    sb_client.subscribe(
        "open-position-comand".into(),
        "position-manager".into(),
        TopicQueueType::PermanentWithSingleConnection,
        Arc::new(OpenPositionSubscriber::new(app.clone())),
    ).await;

    sb_client.subscribe(
        "bid-ask".into(),
        "position-manager".into(),
        TopicQueueType::PermanentWithSingleConnection,
        Arc::new(BidAskSubscriber::new(app.clone()))
    ).await;

    return sb_client;
}
