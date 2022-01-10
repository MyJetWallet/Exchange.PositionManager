use crate::{app_facades::{AccountsCacheFacade, SbEventPublisherFacade}, orders::BidAsk};

pub async fn handle_bid_ask_and_close_positions<T: AccountsCacheFacade, F: SbEventPublisherFacade>(
    account_cache_proxy: T,
    sb_publisher: F,
    bid_ask: BidAsk,
) {
    match account_cache_proxy
        .update_rate_and_close_positions(&bid_ask)
        .await
    {
        Some(closed_orders) => {
            for order in closed_orders {
                sb_publisher.publish_close_position_event(order).await;
            }
        }
        None => {}
    }
}
