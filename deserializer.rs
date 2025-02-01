use serde_json;

pub fn deserialize_from_json<T: serde::de::DeserializeOwned>(data: &str) -> Result<T, String> {
    serde_json::from_str(data).map_err(|_| "Failed to deserialize from JSON".to_string())
}