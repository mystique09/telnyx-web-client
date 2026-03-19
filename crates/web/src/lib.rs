pub mod dto;
pub(crate) mod flash;
pub(crate) mod handlers;
pub(crate) mod inertia;
pub(crate) mod middlewares;
pub mod realtime;
pub mod server;
pub(crate) mod session;
pub mod types;
pub(crate) mod webhook_forwarding;

use serde::{Serialize, Serializer, ser::SerializeMap};

pub struct Empty;

impl Serialize for Empty {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let map = serializer.serialize_map(Some(0))?;
        map.end()
    }
}
