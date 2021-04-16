use crate::libs::ctx::Context;
use tide::{Request, StatusCode};
use hyper::header::Headers;
use std::io::{self, Read, Cursor};

pub async fn read_multipart_data(req: &mut Request<Context>) -> tide::Result<formdata::FormData> {
    let err = || tide::Error::from_str(StatusCode::BadRequest, "Header Parse Failed");
    let content_type = req.content_type().ok_or_else(err)?;
    let content_type = format!("{}", content_type);
    let content_length = req.header("Content-Length").ok_or_else(err)?;
    let content_length = content_length.to_string();
    let data = req.body_bytes().await?;

    let mut stream = MockStream::new(data);
    let mut headers = Headers::new();
    headers.append_raw("Content-Type", content_type.into_bytes());
    headers.append_raw("Content-Length", content_length.into_bytes());

    let err = |_| tide::Error::from_str(StatusCode::BadRequest, "Multipart Form Parse Failed");
    formdata::read_formdata(&mut stream, &headers).map_err(err)
}

struct MockStream {
    pub read: Cursor<Vec<u8>>,
}

impl MockStream {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            read: Cursor::new(data),
        }
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read.read(buf)
    }
}
