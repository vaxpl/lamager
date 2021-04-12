use serde::Serialize;

#[derive(Serialize)]
pub struct ApiMessage<D, E, M> {
    pub data: Option<D>,
    pub errors: Option<E>,
    pub meta: Option<M>,
}
