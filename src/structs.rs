use druid::{Data,Lens};
use std::sync::Arc;


#[derive(Clone, Data, Lens)]
pub struct DefaultState {
    pub name: String,
    pub items: Arc<Vec<String>>
}
