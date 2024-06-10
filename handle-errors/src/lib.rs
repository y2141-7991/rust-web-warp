use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

use reqwest::Error as ReqwestError;
use reqwest_middleware::Error as MiddlewareReqwestError;
use tracing::{event, instrument, Level};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DataBaseQueryError,
    ReqwestAPIError(ReqwestError),
    MiddlewareReqwestAPIError(MiddlewareReqwestError),
    ClientError(APILayerError),
    ServerError(APILayerError),
}

#[derive(Debug, Clone)]
pub struct APILayerError {
    pub status: u16,
    pub message: String,
}

impl std::fmt::Display for APILayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Status: {}, Message: {}", self.status, self.message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &*self {
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            }
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::DataBaseQueryError => {
                write!(f, "Cannot update, invalid data.")
            }
            Error::ReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            }
            Error::MiddlewareReqwestAPIError(err) => {
                write!(f, "External API error: {}", err)
            }
            Error::ClientError(err) => {
                write!(f, "External Client error: {}", err)
            }
            Error::ServerError(err) => {
                write!(f, "External Server error: {}", err)
            }
        }
    }
}

impl Reject for Error {}

#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        event!(Level::ERROR, "{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(crate::Error::ReqwestAPIError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal server error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::MiddlewareReqwestAPIError(e)) =
        r.find()
    {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal server error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::ClientError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal server error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(crate::Error::ServerError(e)) = r.find() {
        event!(Level::ERROR, "{}", e);
        Ok(warp::reply::with_status(
            "Internal server error".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS forbidden error: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        event!(
            Level::ERROR,
            "Can not deserialize request body: {}",
            error
        );
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(crate::Error::DataBaseQueryError) = r.find() {
        event!(Level::ERROR, "Database query error");
        Ok(warp::reply::with_status(
            crate::Error::DataBaseQueryError.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
