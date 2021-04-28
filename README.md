
# Email Inbound Webhook

Receive emails via [SendGrid inbound email parse webhook](https://sendgrid.com/docs/for-developers/parsing-email/setting-up-the-inbound-parse-webhook/). Then
you can query them for automation test or email content parsing.

## Installation

```bash
cargo install --git https://github.com/shaoyanmin/email-inbound-webhook.git 
```

## Environment Variables & CLI Arguments

To run this project, you will need to add the following environment variables:

`DATABASE_URL` **required**

`SUFFIX_FILTER_EMAIL_TO` **required**

`PORT` *optional*

`LOG_LEVEL` *optional*

Or you can override these configs with cli arguments, try `-h` for more information.

## API Reference

#### Query Received Emails in Ascending Order

```http
  GET /emails?to=<EMAIL>&created_after=<CREATED_AFTER>
  
  Responses: 
  
      200 OK
      { "emails": [] }
  
      400 BadRequest
```

| Query Parameter | Type     | Description                |
| :-------- | :------- | :------------------------- |
| `<EMAIL>` | `string` | **Required**. emails' destination address |
| `<CREATED_AFTER>` | `number` | **Required** emails received after this timestamp, e.g., JavaScript `Date.now()` |

#### SendGrid Inbound Parse Webhook Payload

```http
  POST /sendgrid
  
  Responses: 
  
      200 OK
      
      400 BadRequest
```

| Multipart Form | Type     | Description                       |
| :-------- | :------- | :-------------------------------- |
| `from`      | `string` | **Required**. Email sender |
| `to`      | `string` | **Required**. Email receiver |
| `text`      | `string` | **Required**. Text body of email |
| `html`      | `string` | **Required**. HTML body of email |
|...|||

[more](https://sendgrid.com/docs/for-developers/parsing-email/setting-up-the-inbound-parse-webhook/#example-default-payload)

## Deployment

Prerequisites:
* Register a SendGrid account
* Own a domain name

For more details, see [SendGrid Inbound Parse Webhook setup documents](https://sendgrid.com/docs/for-developers/parsing-email/setting-up-the-inbound-parse-webhook/#setup).

Nginx config examples
```text
server {
  listen 8080;
  location / {
    proxy_pass http://localhost:3200;
    proxy_http_version  1.1; # IMPORTANT, hyper only support http 1.1
    proxy_set_header X-Real-IP         $remote_addr;
    proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
  }
}
```

## License

[MIT](https://choosealicense.com/licenses/mit/)

  
