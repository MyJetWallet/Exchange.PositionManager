use std::sync::Arc;

use my_service_bus_shared::queue::TopicQueueType;

use crate::{
    app::AppContext,
    flows::{handle_open_position},
    service_bus::OpenPositionCommand,
};

pub async fn open_position_subscribe(
    app: Arc<AppContext>,
    sb_client: &my_service_bus_tcp_client::MyServiceBusClient,
) {
    let mut subscriber = sb_client
        .subscribe(
            "open_position".into(),
            "position_manager".into(),
            TopicQueueType::Permanent,
        )
        .await;

    let sub_handle = tokio::spawn(async move {
        println!("Open position subscribe started");
        for mess in subscriber.get_next_messages().await {
            match OpenPositionCommand::parse(&mess.content[..]) {
                Ok(command) => {
                    let open_result = handle_open_position(app.clone(), command).await;
                }
                Err(error_mess) => {
                    panic!("Error handle bid ask: {}", error_mess)
                }
            };
        }
        //println!("subscriber finished!");
    });

    // app.active_orders_cache.update_rate_and_close_positions(bid_ask);

}
