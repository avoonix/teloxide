use serde::{Deserialize, Serialize};
use url::Url;

use super::{PhotoSize, UserId};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer_lerge_media: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

impl LinkPreviewOptions {
    pub fn new() -> Self
    {
        Self { is_disabled: None, prefer_lerge_media: None, prefer_small_media: None, show_above_text: None, url: None }
    }

    pub fn prefer_small_media<S>(mut self, val: S) -> Self
    where
        S: Into<bool>,
    {
        self.prefer_small_media = Some(val.into());
        self
    }

    pub fn show_above_text<S>(mut self, val: S) -> Self
    where
        S: Into<bool>,
    {
        self.show_above_text = Some(val.into());
        self
    }

    pub fn is_disabled<S>(mut self, val: S) -> Self
    where
        S: Into<bool>,
    {
        self.is_disabled = Some(val.into());
        self
    }

    pub fn url<S>(mut self, val: S) -> Self
    where
        S: Into<Url>,
    {
        self.url = Some(val.into());
        self
    }
}
