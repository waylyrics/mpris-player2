extern crate dbus;
extern crate glib;
extern crate gtk;

mod mpris_player;
pub use mpris_player::MprisPlayer as MprisPlayer;
use mpris_player::TData;

mod metadata;
pub use metadata::Metadata as Metadata;

mod status;
pub use status::PlaybackStatus as PlaybackStatus;
pub use status::LoopStatus as LoopStatus;

mod generated;
use generated::mediaplayer2::OrgMprisMediaPlayer2 as OrgMprisMediaPlayer2;
use generated::mediaplayer2_player::OrgMprisMediaPlayer2Player as OrgMprisMediaPlayer2Player;
use generated::mediaplayer2::org_mpris_media_player2_server;
use generated::mediaplayer2_player::org_mpris_media_player2_player_server;

use dbus::{Connection, BusType, tree, Message};
use dbus::tree::{Interface, MTFn};
use std::sync::Arc;

fn main() {
    let c = glib::MainContext::default();
    let mainloop = glib::MainLoop::new(Some(&c), false);
    gtk::init();

    let mpris_player = MprisPlayer::new("podcasts".to_string(), "GNOME Podcasts".to_string(), "org.gnome.Podcasts.desktop".to_string());
    mpris_player.set_playback_status(PlaybackStatus::Stopped);

    // Initial test metadata
    let mut metadata = Metadata::new();
    metadata.artist = Some(vec!["Rust Demo".to_string()]);
    metadata.title = Some("Test 1!".to_string());
    metadata.art_url = Some("https://gitlab.gnome.org/uploads/-/system/project/avatar/142/podcasts-logo.png".to_string());
    mpris_player.set_metadata(metadata);

    // change something after 4 seconds...
    let mp = mpris_player.clone();
    gtk::timeout_add(4000, move||{
        mp.set_playback_status(PlaybackStatus::Playing);
        let mut metadata = Metadata::new();
        metadata.artist = Some(vec!["Rust Demo".to_string()]);
        metadata.title = Some("4 seconds...!".to_string());
        metadata.art_url = Some("https://gitlab.gnome.org/uploads/-/system/project/avatar/142/podcasts-logo.png".to_string());
        mp.set_metadata(metadata);
        gtk::Continue(false)
    });

    // change something after 8 seconds......
    let mp = mpris_player.clone();
    gtk::timeout_add(8000, move||{
        mp.set_playback_status(PlaybackStatus::Stopped);
        let mut metadata = Metadata::new();
        metadata.artist = Some(vec!["Rust Demo".to_string()]);
        metadata.title = Some("8 seconds...!".to_string());
        metadata.art_url = Some("https://gitlab.gnome.org/uploads/-/system/project/avatar/142/podcasts-logo.png".to_string());
        mp.set_metadata(metadata);
        gtk::Continue(false)
    });

    mainloop.run();
}
