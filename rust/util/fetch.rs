use alloc::{format, vec::Vec};

/// Net fetch error
#[derive(Debug)]
pub enum NetError {
    /// Invalid URL
    InvalidUrl,
    /// Connection failed
    ConnectionFailed,
    /// Write failed
    WriteFailed,
    /// Read failed
    ReadFailed,
    /// Header parse failed
    HeaderParseFailed,
    /// Unsupported
    Unsupported,
}

/// STD fetch for raw data
/// ex. fetch_url("http://example.com/file.bin")
#[cfg(feature = "std")]
pub fn fetch_url(url: &str, headers: &[(&str, &str)]) -> Result<Vec<u8>, NetError> {
    use std::{
        io::{Read, Write},
        net::TcpStream,
    };

    // Very crude parsing — expects "http://host/path"
    let url = url.strip_prefix("http://").ok_or(NetError::InvalidUrl)?;
    let parts: Vec<&str> = url.splitn(2, '/').collect();

    let host = parts.first().ok_or(NetError::InvalidUrl)?;
    let path = parts.get(1).copied().unwrap_or("");

    let addr = format!("{host}:80");
    // let request = format!("GET /{} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
    let mut request = format!("GET /{path} HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n");
    for (k, v) in headers {
        request.push_str(&format!("{k}: {v}\r\n"));
    }
    request.push_str("\r\n");

    let mut stream = TcpStream::connect(addr).map_err(|_| NetError::ConnectionFailed)?;
    stream.write_all(request.as_bytes()).map_err(|_| NetError::WriteFailed)?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).map_err(|_| NetError::ReadFailed)?;

    // Skip headers
    let body_start =
        response.windows(4).position(|w| w == b"\r\n\r\n").ok_or(NetError::HeaderParseFailed)?;

    Ok(response[(body_start + 4)..].to_vec())
}

#[cfg(not(feature = "std"))]
pub fn fetch_url(_url: &str, headers: &[(&str, &str)]) -> Result<Vec<u8>, NetError> {
    Err(NetError::Unsupported)
}
