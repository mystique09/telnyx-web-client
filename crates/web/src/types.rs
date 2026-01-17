use serde::Serialize;

#[derive(serde::Serialize)]
pub(crate) struct DataPage<T>
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
