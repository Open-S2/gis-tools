#![cfg_attr(feature = "nightly", feature(coverage_attribute))]

mod data_store;
mod data_structures;
mod geometry;
mod parsers;
mod proj;
mod readers;
// mod space;
mod tools;
mod util;
mod writers;

use std::{fs, thread};
use tiny_http::{Header, Response, Server};

// #[coverage(off)]
#[cfg_attr(feature = "nightly", coverage(off))]
pub fn spawn_test_server(root: &str) -> String {
    let server = Server::http("0.0.0.0:0").unwrap();
    let addr = server.server_addr();
    let root = root.to_string();

    thread::spawn(move || {
        for req in server.incoming_requests() {
            let url_path = req.url();
            let path = format!("{}{}", root, url_path);
            let data = fs::read(&path);

            match data {
                Ok(bytes) => {
                    // Check for Range header
                    let range_opt = req
                        .headers()
                        .iter()
                        .find(|h| h.field.equiv("Range"))
                        .map(|h| h.value.as_str());

                    if let Some(range_header) = range_opt {
                        // Only handle "bytes=start-end" format
                        if let Some(range_str) = range_header.strip_prefix("bytes=") {
                            let mut parts = range_str.split('-');
                            let start: usize = parts.next().unwrap_or("0").parse().unwrap_or(0);
                            let end: usize = parts
                                .next()
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(bytes.len() - 1);

                            // Clamp to valid file size
                            let start = start.min(bytes.len());
                            let end = end.min(bytes.len() - 1);

                            let chunk = &bytes[start..=end];

                            let resp = Response::from_data(chunk.to_vec())
                                .with_status_code(206)
                                .with_header(
                                    Header::from_bytes(
                                        &b"Content-Range"[..],
                                        format!("bytes {}-{}/{}", start, end, bytes.len()),
                                    )
                                    .unwrap(),
                                )
                                .with_header(
                                    Header::from_bytes(&b"Accept-Ranges"[..], &b"bytes"[..])
                                        .unwrap(),
                                )
                                .with_header(
                                    Header::from_bytes(
                                        &b"Content-Length"[..],
                                        chunk.len().to_string(),
                                    )
                                    .unwrap(),
                                );

                            req.respond(resp).unwrap();
                            continue;
                        }
                    }

                    // No range requested, serve whole file
                    req.respond(Response::from_data(bytes)).unwrap();
                }
                Err(_) => {
                    req.respond(Response::empty(404)).unwrap();
                }
            }
        }
    });

    format!("http://{}", addr)
}
