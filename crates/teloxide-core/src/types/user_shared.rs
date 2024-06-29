use serde::{Deserialize, Serialize};

use super::SharedUser;

/// Information about the chat whose identifier was shared with the bot using a
/// [`KeyboardButtonRequestUsers`] button.
///
/// [`KeyboardButtonRequestUsers`]: crate::types::KeyboardButtonRequestUsers
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct UsersShared {
    /// Identifier of the request.
    pub request_id: i32,
    /// Identifier of the shared user.
    pub users: Vec<SharedUser>,
}
