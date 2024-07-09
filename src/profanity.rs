use reqwest_middleware::ClientBuilder;
use reqwest_retry::{
    policies::ExponentialBackoff, RetryTransientMiddleware,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIResponse {
    message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadWordResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    start: i64,
    end: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

pub async fn check_profanity(
    content: String,
) -> Result<String, handle_errors::Error> {
    let api_key = env::var("APILAYER_API_KEY")
        .expect("Do not have APILAYER_API_KEY");
    let retry_policy =
        ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_charater=*")
        .header("apikey", format!("{}", api_key))
        .body(content)
        .send()
        .await
        .map_err(|e| handle_errors::Error::MiddlewareReqwestAPIError(e))?;

    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(handle_errors::Error::ClientError(err));
        } else {
            let err = transform_error(res).await;
            return Err(handle_errors::Error::ServerError(err));
        }
    }

    match res.json::<BadWordResponse>().await {
        Ok(res) => Ok(res.censored_content),
        Err(e) => Err(handle_errors::Error::ReqwestAPIError(e)),
    }
}

async fn transform_error(
    res: reqwest::Response,
) -> handle_errors::APIError {
    handle_errors::APIError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}
