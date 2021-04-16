use tide::{Request, Response, StatusCode, Error};

use crate::libs::ctx::Context;
use crate::libs::multipart::read_multipart_data;
use crate::email::Email;

pub async fn parse_inbound_payload(mut req: Request<Context>) -> tide::Result {
    let res = Response::new(StatusCode::Ok);
    let remote = req.remote().ok_or(Error::from_str(StatusCode::BadRequest, ""))?;
    let remote = remote.to_owned();
    let body = read_multipart_data(&mut req).await?;
    Email::create(req.state(), body, remote).await?;
    Ok(res)
}
