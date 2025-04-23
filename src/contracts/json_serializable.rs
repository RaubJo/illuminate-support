use serde_json::Value;

pub trait JsonSerializable {
    fn json_serialize(&self) -> Value;
}
