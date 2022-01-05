#[derive(Debug, Clone)]
pub struct PendingOrderStateInfo{
    pub desire_price: f64
}

impl PendingOrderStateInfo{
    pub fn new(desire_price: f64) -> Self{
        return PendingOrderStateInfo{
            desire_price: desire_price
        };
    }

    pub(crate) fn clone(&self) -> Self {
        Self{desire_price: self.desire_price}
    }
}