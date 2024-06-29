use serde::{Deserialize, Serialize};

use super::{PhotoSize, UserId};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SharedUser {
    pub user_id: UserId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub photo: Option<Vec<PhotoSize>>,
}
