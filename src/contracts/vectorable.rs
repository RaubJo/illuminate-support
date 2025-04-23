use serde_json::Value;

pub trait Vectorable {
    fn to_vec(&self) -> Vec<Value>;
}
