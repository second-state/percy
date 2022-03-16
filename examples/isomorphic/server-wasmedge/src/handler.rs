use crate::response;
use anyhow::Result;
use parsed::http::Response;
use std::io::Read;
use wasmedge_wasi_socket::{SocketAddr, TcpStream};

pub fn handle_req(stream: &mut TcpStream, addr: SocketAddr) -> Result<(Response, Option<Vec<u8>>)> {
    let mut buf = [0u8; 1024];
    let mut received_data: Vec<u8> = Vec::new();

    loop {
        let n = stream.read(&mut buf)?;
        received_data.extend_from_slice(&buf[..n]);
        if n < 1024 {
            break;
        }
    }

    let mut bs: parsed::stream::ByteStream = match String::from_utf8(received_data) {
        Ok(s) => s.into(),
        Err(_) => return Ok((response::bad_request(), None)),
    };

    let req = match parsed::http::parse_http_request(&mut bs) {
        Some(req) => req,
        None => return Ok((response::bad_request(), None)),
    };

    println!("{:?} request: {:?} {:?}", addr, req.method, req.path);

    let mut path_split = req.path.split("?");
    let path = path_split.next().unwrap_or("/");
    let query_str = path_split.next().unwrap_or("");
    let query = querystring::querify(&query_str);
    let mut init_count: Option<u32> = None;
    for (k, v) in query {
        if k.eq("init") {
            match v.parse::<u32>() {
                Ok(v) => init_count = Some(v),
                Err(_) => return Ok((response::bad_request(), None)),
            }
        }
    }

    let (res, binary) = if path.starts_with("/static") {
        response::file(&path)
    } else {
        // render page
        response::ssr(&path, init_count)
    }
    .unwrap_or_else(|_| response::internal_error());

    Ok((res, binary))
}
