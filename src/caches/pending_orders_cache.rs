// use std::collections::BTreeMap;
// use tokio::sync::RwLock;

// use crate::orders::Order;

// pub struct  PendingOrdersCache{
//     pub pending_orders: RwLock<BTreeMap<String, BTreeMap<u128, Order>>>
// }

// impl PendingOrdersCache {
//     pub async fn set_pending_order(&self, order: Order){
//         match &order {
//             Order::Active(_, _) => panic!("Cant set active order as pending"),
//             Order::Closed(_, _) => panic!("Cant set closed order as pending"),
//             Order::Pending(pending_state, order_data) => {
//                 let mut write_access = self.pending_orders.write().await;

//                 let account_orders = write_access.get_mut(&order_data.account_id);

//                 match account_orders {
//                     Some(orders) => {

//                         if orders.contains_key(&order_data.id) {
//                             panic!("Somehow pending order alredy exists in cache")
//                         }

//                         orders.insert(order_data.id, order);
//                     },
//                     None => {
//                         let mut value_to_init = BTreeMap::new();

//                         value_to_init.insert(order_data.account_id.clone(), vec![order]);
//                     },
//                 }
//             },
//         }
//     }

//     pub async fn remove_order(&self, account_id: String, order_id: u128) -> Option<Order>{
//         let mut write_access = self.pending_orders.write().await;
//         let account_orders = write_access.get_mut(&account_id);
        
//         match account_orders {
//             Some(orders) => orders.remove(&order_id),
//             None => None,
//         }
//     }
// }