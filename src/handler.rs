#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerCriteria {
    EverythingElse,
    Extensions(&'static [&'static str]),
}

pub trait FileHandler {
    fn criteria(&self) -> HandlerCriteria;
}
