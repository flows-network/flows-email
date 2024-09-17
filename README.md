# A flow function for sending email

## Deploy the flow function

1. Import this repo (or your fork) into flows.network: https://flows.network/flow/new
2. Click on "Advanced" and configure the following settings.

| Configuration  | Value |
| ------------- | ------------- |
| SENDGRID_AUTH_TOKEN  | The Sendgrid API key  |
| SENDGRID_FROM  | The validated FROM email address registered by this Sendgrid account  |
| RUST_LOG | debug |
| PASS_CODE  | The pass code required to auth the caller  |

## Test

Modify the `example.json` as follows.

* add the pass code in the `code` field
* update the `to` field for your email address

Replace the URL in the `curl` command to the webhook in your deployed flow function.

```
curl -X POST --data-binary "@example.json" https://
```

