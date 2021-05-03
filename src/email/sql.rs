use super::model::Email;
use crate::libs::types::{Query, QueryAs};

impl Email {
    pub fn sql_create<'a>(
        to: &'a str,
        from: &'a str,
        subject: &'a str,
        dkim: &'a str,
        text: &'a str,
        html: &'a str,
        sender_ip: &'a str,
        webhook_request_ip: &'a str,
        created_at: f64,
    ) -> Query<'a> {
        sqlx::query(
            r#"
INSERT INTO emails (
    `to`,
    `from`,
    `subject`,
    `dkim`,
    `text`,
    `html`,
    `sender_ip`,
    `webhook_request_ip`,
    `created_at`
)
VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9
)
            "#,
        ).bind(to)
            .bind(from)
            .bind(subject)
            .bind(dkim)
            .bind(text)
            .bind(html)
            .bind(sender_ip)
            .bind(webhook_request_ip)
            .bind(created_at)
    }

    pub fn sql_find_by_sent_to_and_created_after(to: &str, created_after: f64) -> QueryAs<Self> {
        sqlx::query_as(r#"
SELECT * FROM emails
WHERE
    `to`=? AND created_at>?
ORDER BY created_at ASC
LIMIT 5
        "#).bind(to).bind(created_after)
    }
}