use crate::{app_facades::AccountsCacheFacade, orders::BidAsk};

pub async fn handle_bid_ask_and_close_positions<T: AccountsCacheFacade>(
    account_cache_proxy: T,
    bid_ask: BidAsk,
) {
    match account_cache_proxy
        .update_rate_and_close_positions(&bid_ask)
        .await
    {
        Some(closed_orders) => {
            for order in closed_orders {
                let sb_model = order.to_sb_closed_order();

                //TODO add publish
            }
        }
        None => {}
    }
}
