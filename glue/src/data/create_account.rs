use crate::Endpoint;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CreateAccount {
    pub user_name: String,
    pub email: String,
    pub password: String,
}

impl Endpoint for CreateAccount {
    const PATH: &'static str = "/api/create-account";
}
