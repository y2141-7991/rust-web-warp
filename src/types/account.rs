use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Option<AccountId>,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub i32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewAccount {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: AccountId,
    pub nbf: DateTime<Utc>,
}
