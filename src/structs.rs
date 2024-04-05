use druid::{Data,Lens};

#[derive(Clone, Data, Lens)]
pub struct DefaultState {
    pub name: String,
}