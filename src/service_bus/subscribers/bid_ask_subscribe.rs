use std::sync::Arc;

use async_trait::async_trait;
use my_service_bus_tcp_client::subscribers::{MessagesReader, SubscriberCallback};

use crate::{flows::handle_bid_ask_and_close_positions, orders::BidAsk, AppContext};

pub struct BidAskSubscriber {
    app: Arc<AppContext>,
}

impl BidAskSubscriber {
    pub fn new(app: Arc<AppContext>) -> BidAskSubscriber {
        return BidAskSubscriber { app: app };
    }
}

#[async_trait]
impl SubscriberCallback for BidAskSubscriber {
    async fn new_events(&self, mut messages_reader: MessagesReader) {
        for msg in messages_reader.get_messages() {
            match BidAsk::parse(&msg.content[1..]) {
                Ok(bid_ask) => {
                    handle_bid_ask_and_close_positions(self.app.clone(), self.app.clone(), bid_ask)
                        .await
                }
                Err(error_mess) => {
                    panic!("Error handle bid ask: {}", error_mess)
                }
            };
            messages_reader.handled_ok(&msg);
        }
    }
}
