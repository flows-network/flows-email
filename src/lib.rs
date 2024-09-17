use flowsnet_platform_sdk::logger;
use sendgrid::v3::*;
use serde_json::{json, Value};
use std::collections::HashMap;
use webhook_flows::{
    create_endpoint, request_handler,
    route::{options, post, route, RouteError, Router},
    send_response,
};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    create_endpoint().await;
}

#[request_handler]
async fn handler() {
    let mut router = Router::new();
    router
        .insert("/send_email", vec![options(opt), post(send_email)])
        .unwrap();

    if let Err(e) = route(router).await {
        match e {
            RouteError::NotFound => {
                send_response(404, vec![], b"No route matched".to_vec());
            }
            RouteError::MethodNotAllowed => {
                send_response(405, vec![], b"Method not allowed".to_vec());
            }
        }
    }
}

fn get_default_headers() -> Vec<(String, String)> {
    let resp_headers = vec![
        (
            String::from("content-type"),
            String::from("application/json"),
        ),
        (
            String::from("Access-Control-Allow-Origin"),
            String::from("*"),
        ),
        (
            String::from("Access-Control-Allow-Methods"),
            String::from("GET, POST, OPTIONS"),
        ),
        (
            String::from("Access-Control-Allow-Credentials"),
            String::from("true"),
        ),
        (
            String::from("Access-Control-Allow-Headers"),
            String::from("Set-Cookie,cookie,api,Keep-Alive,User-Agent,Content-Type"),
        ),
    ];
    return resp_headers;
}

async fn opt(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    send_response(200, get_default_headers(), "".as_bytes().to_vec());
}

async fn send_email(_headers: Vec<(String, String)>, _qry: HashMap<String, Value>, body: Vec<u8>) {
    logger::init();
    let sg_sender = std::env::var("SENDGRID_FROM").unwrap();
    let sg_api_key = std::env::var("SENDGRID_AUTH_TOKEN").unwrap();
    let passcode = std::env::var("PASS_CODE").unwrap();

    let mut status = true;
    let mut message = "Sent successfully.";

    let json: Value = serde_json::from_slice(&body).unwrap();
    log::info!(
        "Input JSON: {}",
        serde_json::to_string_pretty(&json).unwrap()
    );
    let code = json
        .get("code")
        .expect("Must have pass code")
        .as_str()
        .unwrap();
    if code == passcode {
        send_response(401, get_default_headers(), "".as_bytes().to_vec())
    }

    let mime = json
        .get("mime")
        .expect("Must have MIME type")
        .as_str()
        .unwrap();
    let email = json
        .get("to")
        .expect("Must have a TO email address")
        .as_str()
        .unwrap();
    let subject = json
        .get("subject")
        .expect("Must have subject")
        .as_str()
        .unwrap();
    let body = json
        .get("body")
        .expect("Must have body")
        .as_str()
        .unwrap();

    let p = Personalization::new(Email::new(email));
    let m = Message::new(Email::new(sg_sender))
        .set_subject(&subject)
        .add_content(
            Content::new()
                .set_content_type(mime)
                .set_value(body),
        )
        .add_personalization(p);

    let sender = Sender::new(sg_api_key);
    match sender.send(&m).await {
        Ok(resp) => {
            log::info!("Sendgrid response: {:?}", resp);
        }
        Err(e) => {
            status = false;
            message = "Cannot send email message.";
            log::info!("Sendgrid error: {:?}", e);
        }
    }
    
    let resp_json = json!({
        "status": status,
        "message": message,
    });
    send_response(
        200,
        get_default_headers(),
        serde_json::to_string(&resp_json)
            .unwrap()
            .as_bytes()
            .to_vec(),
    );
}
