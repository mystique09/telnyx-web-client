pub(crate) mod handlers;
pub(crate) mod inertia;
pub mod server;
pub(crate) mod types;

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
