use crate::mime::MimeType;
use anyhow::Result;
use parsed::http::{Header, Response};
use std::fs::{read};
use std::path::Path;
use isomorphic_app::App;

const HTML_PLACEHOLDER: &str = "#HTML_INSERTED_HERE_BY_SERVER#";
const STATE_PLACEHOLDER: &str = "#INITIAL_STATE_JSON#";

pub fn ssr(path: &str, init: Option<u32>) -> Result<(Response, Option<Vec<u8>>)> {
    let html = format!("{}", include_str!("./index.html"));

    let app = App::new(init.unwrap_or(1001), path.to_string());
    let state = app.store.borrow();

    let html = html.replace(HTML_PLACEHOLDER, &app.render().to_string());
    let html = html.replace(STATE_PLACEHOLDER, &state.to_json());

    Ok((Response {
        protocol: "HTTP/1.0".to_string(),
        code: 200,
        message: "OK".to_string(),
        headers: vec![
            Header {
                name: "content-type".to_string(),
                value: MimeType::from_ext("html").get(),
            },
            Header {
                name: "content-length".to_string(),
                value: html.len().to_string(),
            },
        ],
        content: html.into_bytes(),
    }, None))
}

/// Get raw file content
pub fn file(path: &str) -> Result<(Response, Option<Vec<u8>>)> {
    let path = Path::new(&path);
    if path.exists() {
        let content_type: MimeType = match path.extension() {
            Some(ext) => MimeType::from_ext(ext.to_str().get_or_insert("")),
            None => MimeType::from_ext(""),
        };
        let content = read(path)?;

        Ok((Response {
            protocol: "HTTP/1.0".to_string(),
            code: 200,
            message: "OK".to_string(),
            headers: vec![
                Header {
                    name: "content-type".to_string(),
                    value: content_type.get(),
                },
                Header {
                    name: "content-length".to_string(),
                    value: content.len().to_string(),
                },
            ],
            content: vec![],
        }, Some(content)))
    } else {
        Ok((Response {
            protocol: "HTTP/1.0".to_string(),
            code: 404,
            message: "Not Found".to_string(),
            headers: vec![],
            content: vec![],
        }, None))
    }
}

/// Bad Request
pub fn bad_request() -> Response {
    Response {
        protocol: "HTTP/1.0".to_string(),
        code: 400,
        message: "Bad Request".to_string(),
        headers: vec![],
        content: vec![],
    }
}

/// Internal Server Error
pub fn internal_error() -> (Response, Option<Vec<u8>>) {
    (Response {
        protocol: "HTTP/1.0".to_owned(),
        code: 500,
        message: "Internal Server Error".to_owned(),
        headers: vec![],
        content: vec![],
    }, None)
}
