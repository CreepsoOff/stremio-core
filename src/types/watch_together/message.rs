use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{
    ChatEntry, ConnectionStatus, PlaybackState, Room, RoomMember, RoomMode,
    WatchTogetherContentType,
};

/// The type of sync action for playback control.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SyncActionKind {
    Play,
    Pause,
    Seek,
}

// ─── Client → Server ────────────────────────────────────────────────────────

/// Messages sent from the client to the watch-together server.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WTClientMessage {
    CreateRoom {
        content_id: String,
        content_type: WatchTogetherContentType,
        display_name: String,
    },
    JoinRoom {
        room_id: String,
        display_name: String,
    },
    LeaveRoom,
    SyncAction {
        action: SyncActionKind,
        time: u64,
    },
    Heartbeat {
        time: u64,
    },
    ChatMessage {
        text: String,
    },
    Reaction {
        emoji: String,
        time: u64,
    },
    SetMode {
        mode: RoomMode,
    },
}

// ─── Server → Client ────────────────────────────────────────────────────────

/// Messages received from the watch-together server.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WTServerMessage {
    RoomCreated {
        room_id: String,
        invite_code: String,
    },
    RoomState {
        room_id: String,
        members: Vec<RoomMember>,
        playback: PlaybackState,
        mode: RoomMode,
        content_id: String,
        content_type: WatchTogetherContentType,
        chat_history: Vec<ChatEntry>,
    },
    MemberJoined {
        member: RoomMember,
    },
    MemberLeft {
        member_id: String,
    },
    SyncCommand {
        action: SyncActionKind,
        time: u64,
        from: String,
    },
    ChatBroadcast {
        from: String,
        from_name: String,
        text: String,
        ts: DateTime<Utc>,
    },
    ReactionBroadcast {
        from: String,
        from_name: String,
        emoji: String,
        time: u64,
    },
    ModeChanged {
        mode: RoomMode,
        changed_by: String,
    },
    Error {
        code: WTErrorCode,
        message: String,
    },
    DriftCorrection {
        server_time: u64,
    },
}

/// Error codes from the watch-together server.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WTErrorCode {
    RoomNotFound,
    RoomFull,
    NotInRoom,
    NotHost,
    InvalidMessage,
    NameTaken,
}

/// High-level events emitted by the WatchTogether model for consumers.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum WatchTogetherEvent {
    RoomJoined { room: Room },
    RoomLeft,
    ConnectionChanged { status: ConnectionStatus },
    SyncReceived { action: SyncActionKind, time: u64 },
    Error { message: String },
}
