use std::sync::Arc;

use async_trait::async_trait;
use my_service_bus_tcp_client::subscribers::{MessagesReader, SubscriberCallback};

use crate::{flows::handle_open_position, service_bus::OpenPositionCommand, AppContext};

pub struct OpenPositionSubscriber {
    app: Arc<AppContext>,
}

impl OpenPositionSubscriber {
    pub fn new(app: Arc<AppContext>) -> OpenPositionSubscriber {
        return OpenPositionSubscriber { app: app };
    }
}

#[async_trait]
impl SubscriberCallback for OpenPositionSubscriber {
    async fn new_events(&self, mut messages_reader: MessagesReader) {
        for msg in messages_reader.get_messages() {
            match OpenPositionCommand::parse(&msg.content[..]) {
                Ok(command) => {
                    handle_open_position(self.app.clone(), self.app.clone(), command).await;
                }
                Err(error_mess) => {
                    panic!("Error handle bid ask: {}", error_mess)
                }
            }
        }
    }
}
