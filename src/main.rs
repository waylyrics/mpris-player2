extern crate dbus;

mod mpris_player;
mod mediaplayer2;
mod mediaplayer2_player;

use mpris_player::*;
use mediaplayer2::*;
use mediaplayer2_player::*;

use std::sync::Arc;
use dbus::{Connection, BusType, tree};
use dbus::tree::{Interface, MTFnMut};


#[derive(Copy, Clone, Default, Debug)]
struct TData;
impl tree::DataType for TData {
    type Tree = ();
    type ObjectPath = Arc<MprisPlayer>;
    type Property = ();
    type Interface = ();
    type Method = ();
    type Signal = ();
}

fn main() {
    let mpris_player = Arc::new(MprisPlayer::new());
    let f = tree::Factory::new_fnmut();

    // Create OrgMprisMediaPlayer2 interface
    let root_iface: Interface<MTFnMut<TData>, TData> = org_mpris_media_player2_server(&f, (), |m| {
        let a: &Arc<MprisPlayer> = m.path.get_data();
        let b: &MprisPlayer = &a;
        b
    });

    // Create OrgMprisMediaPlayer2Player interface
    let player_iface: Interface<MTFnMut<TData>, TData> = org_mpris_media_player2_player_server(&f, (), |m| {
        let a: &Arc<MprisPlayer> = m.path.get_data();
        let b: &MprisPlayer = &a;
        b
    });

    // Create dbus tree
    let mut tree = f.tree(());
    tree = tree.add(f.object_path("/org/mpris/MediaPlayer2", mpris_player.clone())
        .introspectable()
        .add(root_iface)
        .add(player_iface)
    );

    // Create dbus connection
    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("org.mpris.MediaPlayer2.rust", 0).unwrap();
    tree.set_registered(&c, true).unwrap();

    // Loop and wait for incoming messages
    c.add_handler(tree);
    loop {
        c.incoming(1000).next();
    }
}
