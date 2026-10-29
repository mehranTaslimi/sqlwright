use crate::backend::Backend;

pub trait Table: Copy + Send + Sync + 'static {
    type Backend: Backend;
    const NAME: &'static str;
}
