use reqwest_middleware::ClientBuilder;
use reqwest_retry::{
    policies::ExponentialBackoff, RetryTransientMiddleware,
};
use std::env;

pub async fn check_profanity(content: String) {
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
        .map_err(|e| handle_errors::Error::MiddlewareReqwestAPIError(e));
}
