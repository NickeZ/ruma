//! Types for the `m.room.topic` event.

use ruma_events_macros::EventContent;
use serde::{Deserialize, Serialize};

/// The content of an `m.room.topic` event.
///
/// A topic is a short message detailing what is currently being discussed in the room.
#[derive(Clone, Debug, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[ruma_event(type = "m.room.topic", kind = State)]
pub struct RoomTopicEventContent {
    /// The topic text.
    pub topic: String,
}

impl RoomTopicEventContent {
    /// Creates a new `RoomTopicEventContent` with the given topic.
    pub fn new(topic: String) -> Self {
        Self { topic }
    }
}
