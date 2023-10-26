use crate::InteractionHandler;
use ed25519_dalek::{Signature, Verifier, VerifyingKey, PUBLIC_KEY_LENGTH};
use lambda_http::http::{HeaderMap, StatusCode};
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde_json::json;

pub async fn run_handler<T>(app_pk: &str, handler: &T) -> Result<(), Error>
where
    T: InteractionHandler + Sync,
{
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(|req: Request| async {
        handle_request(req, app_pk, handler).await
    }))
    .await
}

async fn handle_request<T>(req: Request, app_pk: &str, handler: &T) -> Result<Response<Body>, Error>
where
    T: InteractionHandler + Sync,
{
    let req_body = std::str::from_utf8(req.body()).unwrap();
    let headers = req.headers();

    match verify(&req_body, headers, app_pk) {
        Ok(()) => {
            let res_body = handle_body(handler, &req_body).unwrap_or("{}".to_string());

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(res_body.into())
                .unwrap())
        }

        Err(code) => Ok(Response::builder()
            .status(code)
            .body("Error when handling request.".into())
            .unwrap()),
    }
}

fn handle_body<T>(handler: &T, req_json: &str) -> Option<String>
where
    T: InteractionHandler + Sync,
{
    tracing::info!({ %req_json }, "Request JSON");
    
    match serde_json::from_str::<super::discord_types::Request>(req_json) {
        Ok(interaction) => {
            let res = super::handler::handle_interaction(handler, &interaction);

            let res_json = json!(res).to_string();

            tracing::info!({ %res_json }, "Response JSON");

            Some(res_json)
        }

        Err(_) => None,
    }
}

fn verify(body: &str, headers: &HeaderMap, app_pk: &str) -> Result<(), StatusCode> {
    let application_public_key: [u8; PUBLIC_KEY_LENGTH] = hex::decode(&app_pk)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let verifier = VerifyingKey::from_bytes(&application_public_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let timestamp = headers
        .get("X-Signature-Timestamp")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let signature_str = headers
        .get("X-Signature-Ed25519")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let signature: [u8; 64] = hex::decode(&signature_str)
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let msg = (timestamp.to_owned() + body).into_bytes();

    verifier
        .verify(&msg, &Signature::from_bytes(&signature))
        .map_err(|_| StatusCode::UNAUTHORIZED)
}
