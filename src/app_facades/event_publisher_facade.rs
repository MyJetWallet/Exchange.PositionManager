use async_trait::async_trait;

use crate::{orders::{Order}, flows::OpenPositionResult};

#[async_trait]
pub trait SbEventPublisherFacade{
    async fn publish_open_position_event(&self, open_event: OpenPositionResult);
    async fn publish_close_position_event(&self, order: Order);
}