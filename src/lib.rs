extern crate dbus;
extern crate glib;

mod mpris_player;
pub use crate::mpris_player::MprisPlayer as MprisPlayer;

mod metadata;
pub use crate::metadata::Metadata as Metadata;

mod status;
pub use crate::status::PlaybackStatus as PlaybackStatus;
pub use crate::status::LoopStatus as LoopStatus;

mod generated;
pub use crate::generated::mediaplayer2::OrgMprisMediaPlayer2 as OrgMprisMediaPlayer2;
pub use crate::generated::mediaplayer2_player::OrgMprisMediaPlayer2Player as OrgMprisMediaPlayer2Player;
