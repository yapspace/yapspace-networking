use crate::NetClient;

pub mod channel;
pub mod guild;
pub mod message;
pub mod user;

pub struct V10Impl<'a>(pub &'a NetClient);
