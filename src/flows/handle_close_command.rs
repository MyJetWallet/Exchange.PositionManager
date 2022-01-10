use crate::{
    app_facades::{AccountsCacheFacade, SbEventPublisherFacade},
    service_bus::ClosePositionSbCommand,
};

pub async fn handle_close_command<T: AccountsCacheFacade, F: SbEventPublisherFacade>(
    account_cache_proxy: T,
    sb_publisher: F,
    close_command: &ClosePositionSbCommand,
) {
    match account_cache_proxy
        .remove_order(&close_command.asset_pair_id, &close_command.position_id)
        .await
    {
        Some(order_to_remove) => {
            sb_publisher.publish_close_position_event(order_to_remove).await;
        }
        None => {}
    }
}
