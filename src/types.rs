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

#[derive(serde::Serialize)]
pub struct DataPage<T>
where
    T: Serialize,
{
    component: String,
    props: T,
    url: String,
}

impl<T> DataPage<T>
where
    T: Serialize,
{
    pub fn new(component: String, props: T, url: String) -> Self {
        Self {
            component,
            props,
            url,
        }
    }
}
