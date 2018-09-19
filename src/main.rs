extern crate dbus;
#[macro_use]
extern crate dbus_macros;

mod mpris_player;
mod mediaplayer2;
mod mediaplayer2_player;
use mpris_player::*;


use std::sync::Arc;
use dbus::{Connection, BusType, NameFlag};
use dbus::tree::Factory;

fn main() {
    println!("Start...");
    println!("End!");
}

