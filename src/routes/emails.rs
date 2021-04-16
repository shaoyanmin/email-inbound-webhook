use tide::prelude::*;
use tide::{Request, Response, StatusCode};

use crate::email::Email;
use crate::libs::config::CONFIG;
use crate::libs::ctx::Context;

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
struct PageQuery {
    to: String,
    created_after: f64,
}

pub async fn search_latest_emails(req: Request<Context>) -> tide::Result {
    let page: PageQuery = req.query()?;
    let mut emails = vec![];
    let mut res = Response::new(StatusCode::Ok);
    if page.to.len() == 0 || page.created_after <= 0.0 {
        res.set_body(json!({ "emails": emails }));
        return Ok(res);
    }
    if !page.to.ends_with(CONFIG.suffix_filter_to.as_str()) {
        res.set_status(StatusCode::BadRequest);
        return Ok(res);
    }
    emails = Email::find_emails(req.state(), page.to.as_str(), page.created_after).await?;
    res.set_body(json!({ "emails": emails }));
    Ok(res)
}
