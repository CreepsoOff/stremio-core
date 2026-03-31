use serde::Serialize;

use crate::models::common::eq_update;
use crate::models::ctx::Ctx;
use crate::runtime::msg::{Action, ActionWatchTogether, Internal, Msg};
use crate::runtime::{Effects, Env, UpdateWithCtx};
use crate::types::watch_together::*;

/// The WatchTogether model manages the state of a synchronized viewing session.
///
/// It tracks the current room, connection status, chat history, and reactions.
/// The actual WebSocket communication is handled by the environment (web/native),
/// which dispatches Internal messages back to this model.
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchTogether {
    /// The current room state, if connected to one.
    pub room: Option<Room>,
    /// WebSocket connection status.
    pub connection_status: ConnectionStatus,
    /// Recent reactions (kept for UI display, pruned after a few seconds by the frontend).
    pub reactions: Vec<ReactionEntry>,
}

impl<E: Env + 'static> UpdateWithCtx<E> for WatchTogether {
    fn update(&mut self, msg: &Msg, _ctx: &Ctx) -> Effects {
        match msg {
            // ─── User Actions ────────────────────────────────────────
            Msg::Action(Action::WatchTogether(ActionWatchTogether::LeaveRoom)) => {
                let room_effects = eq_update(&mut self.room, None);
                let conn_effects =
                    eq_update(&mut self.connection_status, ConnectionStatus::Disconnected);
                let reaction_effects = eq_update(&mut self.reactions, vec![]);
                // The environment handles closing the WebSocket
                room_effects.join(conn_effects).join(reaction_effects)
            }

            // ─── Internal: Server Messages ───────────────────────────
            Msg::Internal(Internal::WatchTogetherServerMessage(server_msg)) => {
                match server_msg {
                    WTServerMessage::RoomCreated { .. } => {
                        // Room created confirmation — wait for RoomState
                        Effects::none().unchanged()
                    }
                    WTServerMessage::RoomState {
                        room_id,
                        members,
                        playback,
                        mode,
                        content_id,
                        content_type,
                        chat_history,
                    } => {
                        let room = Room {
                            room_id: room_id.clone(),
                            invite_code: self
                                .room
                                .as_ref()
                                .map(|r| r.invite_code.clone())
                                .unwrap_or_default(),
                            content_id: content_id.clone(),
                            content_type: content_type.clone(),
                            mode: mode.clone(),
                            playback: playback.clone(),
                            members: members.clone(),
                            chat_history: chat_history.clone(),
                        };
                        eq_update(&mut self.room, Some(room))
                    }
                    WTServerMessage::MemberJoined { member } => {
                        if let Some(ref mut room) = self.room {
                            if !room.members.iter().any(|m| m.id == member.id) {
                                room.members.push(member.clone());
                                Effects::none()
                            } else {
                                Effects::none().unchanged()
                            }
                        } else {
                            Effects::none().unchanged()
                        }
                    }
                    WTServerMessage::MemberLeft { member_id } => {
                        if let Some(ref mut room) = self.room {
                            let before = room.members.len();
                            room.members.retain(|m| m.id != *member_id);
                            if room.members.len() != before {
                                Effects::none()
                            } else {
                                Effects::none().unchanged()
                            }
                        } else {
                            Effects::none().unchanged()
                        }
                    }
                    WTServerMessage::SyncCommand { action, time, .. } => {
                        if let Some(ref mut room) = self.room {
                            match action {
                                SyncActionKind::Play => {
                                    room.playback.paused = false;
                                    room.playback.time = *time;
                                }
                                SyncActionKind::Pause => {
                                    room.playback.paused = true;
                                    room.playback.time = *time;
                                }
                                SyncActionKind::Seek => {
                                    room.playback.time = *time;
                                }
                            }
                            Effects::none()
                        } else {
                            Effects::none().unchanged()
                        }
                    }
                    WTServerMessage::ChatBroadcast {
                        from,
                        from_name,
                        text,
                        ts,
                    } => {
                        if let Some(ref mut room) = self.room {
                            room.chat_history.push(ChatEntry {
                                from: from.clone(),
                                from_name: from_name.clone(),
                                text: text.clone(),
                                ts: *ts,
                            });
                            // Keep last 100 messages
                            if room.chat_history.len() > 100 {
                                room.chat_history.remove(0);
                            }
                            Effects::none()
                        } else {
                            Effects::none().unchanged()
                        }
                    }
                    WTServerMessage::ReactionBroadcast {
                        from,
                        from_name,
                        emoji,
                        time,
                    } => {
                        self.reactions.push(ReactionEntry {
                            from: from.clone(),
                            from_name: from_name.clone(),
                            emoji: emoji.clone(),
                            time: *time,
                            ts: chrono::Utc::now(),
                        });
                        // Keep last 50 reactions
                        if self.reactions.len() > 50 {
                            self.reactions.remove(0);
                        }
                        Effects::none()
                    }
                    WTServerMessage::ModeChanged { mode, .. } => {
                        if let Some(ref mut room) = self.room {
                            room.mode = mode.clone();
                            Effects::none()
                        } else {
                            Effects::none().unchanged()
                        }
                    }
                    WTServerMessage::DriftCorrection { server_time } => {
                        if let Some(ref mut room) = self.room {
                            room.playback.time = *server_time;
                            Effects::none()
                        } else {
                            Effects::none().unchanged()
                        }
                    }
                    WTServerMessage::Error { .. } => {
                        // Errors are forwarded as events by the environment
                        Effects::none().unchanged()
                    }
                }
            }

            Msg::Internal(Internal::WatchTogetherConnectionChanged(status)) => {
                eq_update(&mut self.connection_status, status.clone())
            }

            Msg::Action(Action::Unload) => {
                let room_effects = eq_update(&mut self.room, None);
                let conn_effects =
                    eq_update(&mut self.connection_status, ConnectionStatus::Disconnected);
                let reaction_effects = eq_update(&mut self.reactions, vec![]);
                room_effects.join(conn_effects).join(reaction_effects)
            }

            _ => Effects::none().unchanged(),
        }
    }
}
