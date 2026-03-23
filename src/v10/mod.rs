use crate::NetClient;

pub mod guild;
pub mod user;

pub struct V10Impl<'a>(pub &'a NetClient);
