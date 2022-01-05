use crate::{service_bus::ClosePositionSbCommand, app_facades::AccountsCacheFacade};

pub async fn handle_close_command<T: AccountsCacheFacade>(account_cache_proxy: T, close_command: &ClosePositionSbCommand) {
    match account_cache_proxy
        .remove_order(&close_command.asset_pair_id, &close_command.position_id)
        .await
    {
        Some(order_to_remove) => {
            let close_sb_contract = order_to_remove.to_sb_closed_order();
            //ToDo add publish
        },
        None => {},
    }
}
