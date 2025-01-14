//! Types for the [`m.poll.response`] event.

use std::{ops::Deref, vec};

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

use crate::{events::relation::Reference, OwnedEventId};

/// The payload for a poll response event.
#[derive(Clone, Debug, Serialize, Deserialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "org.matrix.msc3381.v2.poll.response", alias = "m.poll.response", kind = MessageLike)]
pub struct PollResponseEventContent {
    /// The user's selection.
    #[serde(rename = "org.matrix.msc3381.v2.selections")]
    pub selections: SelectionsContentBlock,

    /// Whether this message is automated.
    #[cfg(feature = "unstable-msc3955")]
    #[serde(
        default,
        skip_serializing_if = "crate::serde::is_default",
        rename = "org.matrix.msc1767.automated"
    )]
    pub automated: bool,

    /// Information about the poll start event this responds to.
    #[serde(rename = "m.relates_to")]
    pub relates_to: Reference,
}

impl PollResponseEventContent {
    /// Creates a new `PollResponseEventContent` that responds to the given poll start event ID,
    /// with the given poll response content.
    pub fn new(selections: SelectionsContentBlock, poll_start_id: OwnedEventId) -> Self {
        Self {
            selections,
            #[cfg(feature = "unstable-msc3955")]
            automated: false,
            relates_to: Reference::new(poll_start_id),
        }
    }
}

/// A block for selections content.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct SelectionsContentBlock(Vec<String>);

impl SelectionsContentBlock {
    /// Whether this `SelectionsContentBlock` is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<String>> for SelectionsContentBlock {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

impl IntoIterator for SelectionsContentBlock {
    type Item = String;
    type IntoIter = vec::IntoIter<String>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<String> for SelectionsContentBlock {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl Deref for SelectionsContentBlock {
    type Target = [String];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
