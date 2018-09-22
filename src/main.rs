extern crate dbus;

mod mpris_player;
pub use mpris_player::MprisPlayer as MprisPlayer;
use mpris_player::TData;

mod metadata;
pub use metadata::Metadata as Metadata;

mod playback_status;
pub use playback_status::PlaybackStatus as PlaybackStatus;

mod generated;
pub use generated::mediaplayer2::OrgMprisMediaPlayer2 as OrgMprisMediaPlayer2;
pub use generated::mediaplayer2_player::OrgMprisMediaPlayer2Player as OrgMprisMediaPlayer2Player;
use generated::mediaplayer2::org_mpris_media_player2_server;
use generated::mediaplayer2_player::org_mpris_media_player2_player_server;

use dbus::{Connection, BusType, tree};
use dbus::tree::{Interface, MTFn};
use std::sync::Arc;

fn main() {
    let mpris_player = MprisPlayer::new();
    MprisPlayer::run(mpris_player);
}
