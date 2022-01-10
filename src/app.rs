use std::sync::Arc;

use async_trait::async_trait;
use my_service_bus_tcp_client::{MyServiceBusClient, MessageToPublish};
use prost::Message;
use tokio::sync::RwLock;

use crate::{
    app_facades::{AccountsCacheFacade, SbEventPublisherFacade},
    caches::ActiveOrdersCache,
    flows::OpenPositionResult,
    orders::{BidAsk, Order},
};

pub struct AppContext {
    active_orders_cache: Arc<ActiveOrdersCache>,
    sb_client: RwLock<Option<MyServiceBusClient>>,
}

impl AppContext {
    pub fn new() -> AppContext {
        AppContext {
            active_orders_cache: Arc::new(ActiveOrdersCache::new()),
            sb_client: RwLock::new(None),
        }
    }

    pub async fn set_sb_client(&self, sb_client: MyServiceBusClient){
        let mut sb_lock = self.sb_client.write().await;
        *sb_lock = Some(sb_client);
    }

    pub fn get_current_bid_ask(&self, asset_id: &String) -> Option<BidAsk> {
        todo!();
    }
}

#[async_trait]
impl SbEventPublisherFacade for Arc<AppContext> {
    async fn publish_open_position_event(&self, open_event: OpenPositionResult) {
        let mut sb_contract = open_event.into_sb_contaract();
        let serialized_message = serialize_to_protobuff_sb_message(&mut sb_contract, 0);

        self.sb_client
            .read()
            .await
            .as_ref()
            .unwrap()
            .publish("open_position_event".into(), MessageToPublish::new(serialized_message))
            .await;
    }
    async fn publish_close_position_event(&self, order: Order) {
        let mut sb_contract = order.to_sb_closed_order();
        let serialized_message = serialize_to_protobuff_sb_message(&mut sb_contract, 0);

        self.sb_client
            .read()
            .await
            .as_ref()
            .unwrap()
            .publish("close_position_event".into(), MessageToPublish::new(serialized_message))
            .await;
    }
}

#[async_trait]
impl AccountsCacheFacade for Arc<AppContext> {
    async fn remove_order(&self, asset_id: &String, order_id: &String) -> Option<Order> {
        self.active_orders_cache
            .remove_order(asset_id, order_id)
            .await
    }

    async fn set_active_order(&self, order: Order) {
        self.active_orders_cache.set_active_order(order).await
    }

    async fn update_rate_and_close_positions(&self, bid_ask: &BidAsk) -> Option<Vec<Order>> {
        self.active_orders_cache
            .update_rate_and_close_positions(bid_ask)
            .await
    }
}

pub fn serialize_to_protobuff_sb_message<T: Message>(message: &mut T, version: u8) -> Vec<u8> {
    let mut serialized_message = vec![version];
    let mut buffer = Vec::<u8>::new();

    prost::Message::encode(message, &mut buffer);
    serialized_message.extend(buffer);

    return serialized_message;
}
