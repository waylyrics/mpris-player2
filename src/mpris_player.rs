extern crate dbus;
use dbus::arg::{Variant, RefArg};
use dbus::{Connection, BusType, tree};
use dbus::tree::{Interface, MTFn, Factory};

use std::collections::HashMap;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;
use std::rc::Rc;

use generated::mediaplayer2::org_mpris_media_player2_server;
use generated::mediaplayer2_player::org_mpris_media_player2_player_server;

use OrgMprisMediaPlayer2Player;
use OrgMprisMediaPlayer2;

use Metadata;
use PlaybackStatus;

pub struct MprisPlayer{
    connection: Rc<RefCell<Connection>>,

    // OrgMprisMediaPlayer2
    can_quit: Cell<bool>,
    fullscreen: Cell<bool>,
    can_set_fullscreen: Cell<bool>,
    can_raise: Cell<bool>,
    has_track_list: Cell<bool>,
    identify: String,
    desktop_entry: String,
    supported_uri_schemes: Vec<String>,
    supported_mime_types: Vec<String>,

    // OrgMprisMediaPlayer2Player
    playback_status: PlaybackStatus,
    loop_status: String,
    rate: Cell<f64>,
    shuffle: Cell<bool>,
    pub metadata: RefCell<Metadata>,
    volume: Cell<f64>,
    position: Cell<i64>,
    minimum_rate: Cell<f64>,
    maximum_rate: Cell<f64>,
    can_go_next: Cell<bool>,
    can_go_previous: Cell<bool>,
    can_play: Cell<bool>,
    can_pause: Cell<bool>,
    can_seek: Cell<bool>,
    can_control: Cell<bool>,
}

impl MprisPlayer{
    pub fn new(identifier: String) -> Arc<Self>{
        let connection = Rc::new(RefCell::new(Connection::get_private(BusType::Session).unwrap()));

        let mpris_player = Arc::new(MprisPlayer{
            connection,

            can_quit: Cell::new(false),
            fullscreen: Cell::new(false),
            can_set_fullscreen: Cell::new(false),
            can_raise: Cell::new(false),
            has_track_list: Cell::new(false),
            identify: "".to_string(),
            desktop_entry: "".to_string(),
            supported_uri_schemes: Vec::new(),
            supported_mime_types: Vec::new(),

            playback_status: PlaybackStatus::Paused,
            loop_status: "".to_string(),
            rate: Cell::new(0_f64),
            shuffle: Cell::new(false),
            metadata: RefCell::new(Metadata::new()),
            volume: Cell::new(0_f64),
            position: Cell::new(0),
            minimum_rate: Cell::new(0_f64),
            maximum_rate: Cell::new(0_f64),
            can_go_next: Cell::new(true),
            can_go_previous: Cell::new(true),
            can_play: Cell::new(true),
            can_pause: Cell::new(true),
            can_seek: Cell::new(false),
            can_control: Cell::new(true),
        });

        let factory: Factory<MTFn<TData>, TData> = Factory::new_fn();

        // Create OrgMprisMediaPlayer2 interface
        let root_iface: Interface<MTFn<TData>, TData> = org_mpris_media_player2_server(&factory, (), |m| {
            let a: &Arc<MprisPlayer> = m.path.get_data();
            let b: &MprisPlayer = &a;
            b
        });

        // Create OrgMprisMediaPlayer2Player interface
        let player_iface: Interface<MTFn<TData>, TData> = org_mpris_media_player2_player_server(&factory, (), |m| {
            let a: &Arc<MprisPlayer> = m.path.get_data();
            let b: &MprisPlayer = &a;
            b
        });

        // Create dbus tree
        let mut tree = factory.tree(());
        tree = tree.add(factory.object_path("/org/mpris/MediaPlayer2", mpris_player.clone())
            .introspectable()
            .add(root_iface)
            .add(player_iface)
        );

        // Setup dbus connection
        mpris_player.connection.borrow().register_name(&format!("org.mpris.MediaPlayer2.{}", identifier), 0).unwrap();
        tree.set_registered(&mpris_player.connection.borrow(), true).unwrap();
        mpris_player.connection.borrow().add_handler(tree);

        mpris_player
    }

    pub fn run(&self){
        // Wait for incoming messages
        loop {
            let connection = self.connection.clone();
            connection.borrow().incoming(1000).next();
        }
    }
}

impl OrgMprisMediaPlayer2 for MprisPlayer {
    type Err = tree::MethodErr;

    fn raise(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn quit(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_can_quit(&self) -> Result<bool, Self::Err> {
        Ok(self.can_quit.get())
    }

    fn get_fullscreen(&self) -> Result<bool, Self::Err> {
        Ok(self.fullscreen.get())
    }

    fn set_fullscreen(&self, value: bool) -> Result<(), Self::Err> {
        self.fullscreen.set(value);
        Ok(())
    }

    fn get_can_set_fullscreen(&self) -> Result<bool, Self::Err> {
        Ok(self.can_set_fullscreen.get())
    }

    fn get_can_raise(&self) -> Result<bool, Self::Err> {
        Ok(self.can_raise.get())
    }

    fn get_has_track_list(&self) -> Result<bool, Self::Err> {
        Ok(self.has_track_list.get())
    }

    fn get_identity(&self) -> Result<String, Self::Err> {
        Ok(self.identify.clone())
    }

    fn get_desktop_entry(&self) -> Result<String, Self::Err> {
        Ok(self.desktop_entry.clone())
    }

    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, Self::Err> {
        Ok(self.supported_uri_schemes.clone())
    }

    fn get_supported_mime_types(&self) -> Result<Vec<String>, Self::Err> {
        Ok(self.supported_mime_types.clone())
    }
}

impl OrgMprisMediaPlayer2Player for MprisPlayer {
    type Err = tree::MethodErr;

    fn next(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn previous(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn pause(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn play_pause(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn stop(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn play(&self) -> Result<(), Self::Err> {
        Ok(())
    }

    fn seek(&self, offset: i64) -> Result<(), Self::Err> {
        Ok(())
    }

    fn set_position(&self, track_id: dbus::Path, position: i64) -> Result<(), Self::Err> {
        self.position.set(position);
        Ok(())
    }

    fn open_uri(&self, uri: &str) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_playback_status(&self) -> Result<String, Self::Err> {
        Ok(self.playback_status.value())
    }

    fn get_loop_status(&self) -> Result<String, Self::Err> {
        Ok(self.loop_status.clone())
    }

    fn set_loop_status(&self, value: String) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_rate(&self) -> Result<f64, Self::Err> {
        Ok(self.rate.get())
    }

    fn set_rate(&self, value: f64) -> Result<(), Self::Err> {
        self.rate.set(value);
        Ok(())
    }

    fn get_shuffle(&self) -> Result<bool, Self::Err> {
        Ok(self.shuffle.get())
    }

    fn set_shuffle(&self, value: bool) -> Result<(), Self::Err> {
        self.shuffle.set(value);
        Ok(())
    }

    fn get_metadata(&self) -> Result<HashMap<String, Variant<Box<RefArg + 'static>>>, Self::Err> {
        let metadata = self.metadata.borrow().to_hashmap();
        Ok(metadata)
    }

    fn get_volume(&self) -> Result<f64, Self::Err> {
        Ok(self.volume.get())
    }

    fn set_volume(&self, value: f64) -> Result<(), Self::Err> {
        self.volume.set(value);
        Ok(())
    }

    fn get_position(&self) -> Result<i64, Self::Err> {
        Ok(self.position.get())
    }

    fn get_minimum_rate(&self) -> Result<f64, Self::Err> {
        Ok(self.minimum_rate.get())
    }

    fn get_maximum_rate(&self) -> Result<f64, Self::Err> {
        Ok(self.maximum_rate.get())
    }

    fn get_can_go_next(&self) -> Result<bool, Self::Err> {
        Ok(self.can_go_next.get())
    }

    fn get_can_go_previous(&self) -> Result<bool, Self::Err> {
        Ok(self.can_go_previous.get())
    }

    fn get_can_play(&self) -> Result<bool, Self::Err> {
        Ok(self.can_play.get())
    }

    fn get_can_pause(&self) -> Result<bool, Self::Err> {
        Ok(self.can_pause.get())
    }

    fn get_can_seek(&self) -> Result<bool, Self::Err> {
        Ok(self.can_seek.get())
    }

    fn get_can_control(&self) -> Result<bool, Self::Err> {
        Ok(self.can_control.get())
    }
}

impl ::std::fmt::Debug for MprisPlayer{
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result  {
        write!(f, "mprisplayer")
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct TData;
impl tree::DataType for TData {
    type Tree = ();
    type ObjectPath = Arc<MprisPlayer>;
    type Property = ();
    type Interface = ();
    type Method = ();
    type Signal = ();
}
