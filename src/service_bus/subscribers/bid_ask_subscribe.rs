use std::sync::Arc;

use crate::{flows::handle_bid_ask_and_close_positions, orders::BidAsk, AppContext};
use my_service_bus_shared::queue::TopicQueueType;

pub async fn bid_ask_subscribe(
    app: Arc<AppContext>,
    sb_client: &my_service_bus_tcp_client::MyServiceBusClient,
){
    let mut subscriber = sb_client
        .subscribe(
            "bidask".into(),
            "position_manager".into(),
            TopicQueueType::Permanent,
        )
        .await;

    tokio::spawn(async move {
        println!("Bid ask subscribe started");
        for mess in subscriber.get_next_messages().await {
            match BidAsk::parse(&mess.content[1..]) {
                Ok(bid_ask) => handle_bid_ask_and_close_positions(app.clone(), bid_ask).await,
                Err(error_mess) => {
                    panic!("Error handle bid ask: {}", error_mess)
                }
            };
        }
    });
}
