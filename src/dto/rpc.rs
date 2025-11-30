use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub result: Option<T>,
    pub error: Option<ApiError>,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    #[serde(rename = "loginStatus")]
    pub login_status: String,
}

// Alternative login response format (for cert login)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertLoginResponse {
    pub session_token: String,
    pub login_status: String,
}

// Interactive login response format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveLoginResponse {
    #[serde(rename = "sessionToken")]
    pub session_token: Option<String>,
    pub token: Option<String>,
    #[serde(rename = "loginStatus")]
    pub login_status: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "statusCode")]
    pub status_code: Option<String>,
    pub error: Option<String>,
    #[serde(rename = "errorDetails")]
    pub error_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    #[serde(deserialize_with = "deserialize_error_code")]
    pub code: String,
    pub message: String,
}

fn deserialize_error_code<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(s),
        Value::Number(num) => Ok(num.to_string()),
        other => Err(serde::de::Error::custom(format!(
            "unexpected type for Betfair API error code: {other}"
        ))),
    }
}
