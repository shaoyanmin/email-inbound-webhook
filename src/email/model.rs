use chrono::prelude::Utc;
use formdata::FormData;
use tide::http::StatusCode;
use std::collections::HashMap;
use crate::libs::ctx::Context;
use crate::libs::config::CONFIG;

#[derive(serde::Deserialize, serde::Serialize, Debug, sqlx::FromRow, Default)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub dkim: String,
    pub text: String,
    pub html: String,
    pub sender_ip: String,
    pub webhook_request_ip: String,
    pub created_at: f64,
}

impl Email {
    pub async fn create(ctx: &Context, form: FormData, webhook_request_ip: String) -> tide::Result<Self> {
        let n = 7;
        let err = || tide::Error::from_str(StatusCode::BadRequest, "Invalid Form Data");
        if form.fields.len() < n {
            return Err(err());
        }
        let mut map: HashMap<String, String> = form.fields.into_iter().collect();
        let to= map.remove("to").ok_or_else(err)?;
        let from= map.remove("from").ok_or_else(err)?;
        let text= map.remove("text").ok_or_else(err)?;
        let html= map.remove("html").ok_or_else(err)?;
        let sender_ip= map.remove("sender_ip").ok_or_else(err)?;
        let dkim= map.remove("dkim").ok_or_else(err)?;
        let subject= map.remove("subject").ok_or_else(err)?;
        let created_at = Utc::now().timestamp_millis() as f64;

        if !to.ends_with(CONFIG.suffix_filter_to.as_str()) {
            warn!("Ignore inbound email for mismatched `to`: {}, sub: {}", to, subject);
            return Ok(Default::default())
        }

        Self::sql_create(
            to.as_str(),
            from.as_str(),
            subject.as_str(),
            dkim.as_str(),
            text.as_str(),
            html.as_str(),
            sender_ip.as_str(),
            webhook_request_ip.as_str(),
            created_at,
        ).execute(&ctx.db).await.unwrap();

        info!("Save inbound Email to: {}, sub: {}", to, subject);

        Ok(Self { to, from, text, html, sender_ip, webhook_request_ip, dkim, subject, created_at })
    }

    pub async fn find_emails(ctx: &Context, to: &str, created_after: f64) -> tide::Result<Vec<Self>> {
        let query = Self::sql_find_by_sent_to_and_created_after(to, created_after);
        let ans = query.fetch_all(&ctx.db).await.unwrap();
        Ok(ans)
    }
}
