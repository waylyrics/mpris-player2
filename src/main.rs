extern crate dbus;
#[macro_use]
extern crate dbus_macros;

mod mpris_player;
mod mediaplayer2;
mod mediaplayer2_player;
use mpris_player::*;


use std::sync::Arc;
use dbus::tree::Factory;

use mediaplayer2::*;
use mediaplayer2_player::*;


use dbus::{Connection, BusType, tree, Path};
use dbus::tree::{Interface, MTFn, MethodErr};


use std::sync::mpsc;
use std::cell::Cell;
use std::thread;

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

fn create_root_iface() -> Interface<MTFn<TData>, TData> {
    let f = tree::Factory::new_fn();
    org_mpris_media_player2_server(&f, (), |m| {
        let a: &Arc<MprisPlayer> = m.path.get_data();
        let b: &MprisPlayer = &a;
        b
    })
}

fn create_player_iface() -> Interface<MTFn<TData>, TData> {
    let f = tree::Factory::new_fn();
    org_mpris_media_player2_player_server(&f, (), |m| {
        let a: &Arc<MprisPlayer> = m.path.get_data();
        let b: &MprisPlayer = &a;
        b
    })
}

fn create_tree(mpris_player: &Arc<MprisPlayer>, root_iface: &Arc<Interface<MTFn<TData>, TData>>, player_iface: &Arc<Interface<MTFn<TData>, TData>>)
    -> tree::Tree<MTFn<TData>, TData> {

    let f = tree::Factory::new_fn();
    let mut tree = f.tree(());

    tree = tree.add(f.object_path("/org/mpris/MediaPlayer2", mpris_player.clone())
        .introspectable()
        .add(root_iface.clone())
        .add(player_iface.clone())
    );

    tree
}

fn main() {
    println!("Start...");
    let mpris_player = Arc::new(MprisPlayer::new());

    let root_iface = create_root_iface();
    let player_iface = create_player_iface();
    let tree = create_tree(&mpris_player, &Arc::new(root_iface), &Arc::new(player_iface));

    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("org.mpris.MediaPlayer2.rust", 0);
    tree.set_registered(&c, true);

    c.add_handler(tree);
    loop {
        c.incoming(1000).next();
    }

    println!("End!");
}

