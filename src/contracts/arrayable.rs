use serde_json::Value;

pub trait Arrayable{
    fn to_array(&self) -> Vec<Value>;
}
