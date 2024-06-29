use serde::{Deserialize, Serialize};

/// This object defines the criteria used to request a suitable user. The
/// identifier of the selected user will be shared with the bot when the
/// corresponding button is pressed. More about requesting users »
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyboardButtonRequestUsers {
    /// identifier of the request, which will be received back in the
    /// [`UsersShared`] object. Must be unique within the message.
    ///
    /// [`UsersShared`]: crate::types::UsersShared
    pub request_id: i32,

    /// Pass `true` to request a bot, pass `false` to request a regular user. If
    /// not specified, no additional restrictions are applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,

    /// Pass `true` to request a premium user, pass `false` to request a
    /// non-premium user. If not specified, no additional restrictions are
    /// applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

impl KeyboardButtonRequestUsers {
    /// Creates a new [`KeyboardButtonRequestUsers`].
    pub fn new(request_id: i32) -> Self {
        Self {
            request_id,
            user_is_bot: None,
            user_is_premium: None,
            request_name: None,
            request_username: None,
            request_photo: None,
        }
    }

    /// Setter for `user_is_bot` field
    pub fn user_is_bot(mut self, value: bool) -> Self {
        self.user_is_bot = Some(value);
        self
    }

    /// Setter for `user_is_premium` field
    pub fn user_is_premium(mut self, value: bool) -> Self {
        self.user_is_premium = Some(value);
        self
    }

    #[must_use]
    pub fn request_name(mut self, value: bool) -> Self {
        self.request_name = Some(value);
        self
    }
    #[must_use]
    pub fn request_username(mut self, value: bool) -> Self {
        self.request_username = Some(value);
        self
    }
    #[must_use]
    pub fn request_photo(mut self, value: bool) -> Self {
        self.request_photo = Some(value);
        self
    }
}
