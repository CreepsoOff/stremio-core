use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// The mode controlling who can issue playback commands in the room.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RoomMode {
    /// Only the host can control playback.
    Host,
    /// All members can control playback.
    Democracy,
}

impl Default for RoomMode {
    fn default() -> Self {
        Self::Host
    }
}

/// The type of content being watched together.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WatchTogetherContentType {
    Movie,
    Series,
    Anime,
    Other,
}

/// Information about a member in the room.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomMember {
    pub id: String,
    pub display_name: String,
    pub is_host: bool,
}

/// The current playback state of the room.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackState {
    pub paused: bool,
    /// Current playback time in milliseconds.
    pub time: u64,
    pub last_update: DateTime<Utc>,
}

impl Default for PlaybackState {
    fn default() -> Self {
        Self {
            paused: true,
            time: 0,
            last_update: Utc::now(),
        }
    }
}

/// A chat message entry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEntry {
    pub from: String,
    pub from_name: String,
    pub text: String,
    pub ts: DateTime<Utc>,
}

/// A reaction on the video.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReactionEntry {
    pub from: String,
    pub from_name: String,
    pub emoji: String,
    /// Video timestamp at which the reaction was sent (ms).
    pub time: u64,
    pub ts: DateTime<Utc>,
}

/// The full state of a watch-together room.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    pub room_id: String,
    pub invite_code: String,
    pub content_id: String,
    pub content_type: WatchTogetherContentType,
    pub mode: RoomMode,
    pub playback: PlaybackState,
    pub members: Vec<RoomMember>,
    pub chat_history: Vec<ChatEntry>,
}

/// Connection status for the watch-together WebSocket.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self::Disconnected
    }
}
