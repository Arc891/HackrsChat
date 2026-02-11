use crate::protocol::{Request, Response};

pub fn encode_request(request: &Request) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string(request)?;
    json.push('\n');
    Ok(json)
}

pub fn encode_response(response: &Response) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string(response)?;
    json.push('\n');
    Ok(json)
}

pub fn decode_request(s: &str) -> Result<Request, serde_json::Error> {
    serde_json::from_str(s.trim())
}

pub fn decode_response(s: &str) -> Result<Response, serde_json::Error> {
    serde_json::from_str(s.trim())
}
