#![allow(deprecated)]

mod mpris_player;
pub use crate::mpris_player::MprisPlayer;

mod metadata;
pub use crate::metadata::Metadata;

mod status;
pub use crate::status::LoopStatus;
pub use crate::status::PlaybackStatus;

mod generated;
pub use crate::generated::mediaplayer2::OrgMprisMediaPlayer2;
pub use crate::generated::mediaplayer2_player::OrgMprisMediaPlayer2Player;
