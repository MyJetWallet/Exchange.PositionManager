use crate::{service_bus::OpenPositionCommand, OpenPositonSbMessage, app_facades::{AccountsCacheFacade, SbEventPublisherFacade}};

pub struct OpenPositionResult {
    pub id: Option<String>,
    pub request_id: String,
    pub open_date: Option<u64>,
    pub wallet_id: String,
    pub status: OpenPositionStatus,
}

#[derive(Clone, PartialEq, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpenPositionStatus {
    Ok = 1,
    NoLiquidity = 2,
}

impl OpenPositionResult {
    pub fn new(
        id: Option<String>,
        request_id: String,
        wallet_id: String,
        open_date: Option<u64>,
        status: OpenPositionStatus,
    ) -> OpenPositionResult {
        OpenPositionResult {
            id,
            request_id,
            open_date,
            wallet_id,
            status,
        }
    }

    pub fn into_sb_contaract(self) -> OpenPositonSbMessage {
        OpenPositonSbMessage {
            wallet_id: self.wallet_id,
            id: self.id,
            request_id: self.request_id,
            open_date: self.open_date,
            status: self.status as i32,
        }
    }
}

pub async fn handle_open_position<T: AccountsCacheFacade, F: SbEventPublisherFacade>(
    account_cache: T,
    sb_publisher: F,
    position: OpenPositionCommand,
) {
    let current_bid_ask = None;
    let result = match current_bid_ask {
        Some(bid_ask) => {
            let open_price = position.get_order_type().get_open_price(&bid_ask);
            let req_id = position.request_id.clone();
            let active_order = position.to_active_order(bid_ask, open_price);

            let open_result = OpenPositionResult::new(
                Some(active_order.get_id()),
                req_id,
                active_order.get_wallet(),
                Some(active_order.get_open_date()),
                OpenPositionStatus::Ok,
            );

            account_cache.set_active_order(active_order).await;
            open_result
        }
        None => {
            OpenPositionResult::new(
                None,
                position.request_id,
                position.wallet_id,
                None,
                OpenPositionStatus::NoLiquidity,
            )
        }
    };
    sb_publisher.publish_open_position_event(result);

}