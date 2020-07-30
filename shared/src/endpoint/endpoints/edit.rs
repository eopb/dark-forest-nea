pub mod init;
pub mod save;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectPath {
    pub user_name: String,
    pub project_name: String,
}
