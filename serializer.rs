use serde_json;

pub fn serialize_to_json<T: serde::Serialize>(data: &T) -> Result<String, String> {
    serde_json::to_string(data).map_err(|_| "Failed to serialize to JSON".to_string())
}