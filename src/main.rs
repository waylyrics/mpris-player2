extern crate dbus;
extern crate glib;

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
    let mpris_player = MprisPlayer::new("podcasts".to_string(), "GNOME Podcasts".to_string(), "org.gnome.Podcasts.desktop".to_string());

    mpris_player.set_playback_status(PlaybackStatus::Playing);

    let mut metadata = Metadata::new();
    metadata.artist.push("Rust".to_string());
    metadata.artist.push("MPRIS Server".to_string());
    metadata.title = "Is working!".to_string();
    metadata.art_url = "https://gitlab.gnome.org/uploads/-/system/project/avatar/142/podcasts-logo.png".to_string();
    mpris_player.set_metadata(metadata);

    mpris_player.run();
}
