use alloc::{
    string::{String, ToString},
    vec::Vec,
};

/// Net fetch error
#[derive(Debug, Clone, PartialEq)]
pub enum NetError {
    /// Network error
    Network(String),
    /// HTTP error
    Http(u16),
    /// Other
    Other(String),
}

/// STD fetch for raw data
/// ex. `fetch_url("http://example.com/file.bin", &[("header", "value")])`
#[cfg(feature = "std")]
pub async fn fetch_url(url: &str, headers: &[(&str, &str)]) -> Result<Vec<u8>, NetError> {
    let client = surf::client();

    let mut req = surf::get(url);
    for (k, v) in headers {
        req = req.header(*k, *v);
    }

    let mut res = client.send(req).await.map_err(|e| NetError::Network(e.to_string()))?;

    if !res.status().is_success() {
        return Err(NetError::Http(res.status().into()));
    }

    res.body_bytes().await.map_err(|e| NetError::Other(e.to_string()))
}

/// WASM fetch for raw data
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub async fn fetch_url(url: &str, headers: &[(&str, &str)]) -> Result<Vec<u8>, NetError> {
    let mut req = surf::get(url);
    for (k, v) in headers {
        req = req.set_header(*k, *v);
    }

    let mut res = req.await.map_err(|e| NetError::Network(e.to_string()))?;

    if !res.status().is_success() {
        return Err(NetError::Http(res.status().into()));
    }

    res.body_bytes().await.map_err(|e| NetError::Other(e.to_string()))
}
