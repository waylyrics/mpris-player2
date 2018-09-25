extern crate glib;
extern crate gtk;
extern crate dbus;
use dbus::arg::{Variant, RefArg};
use dbus::{Connection, BusType, tree, Path, SignalArgs};
use dbus::tree::{Interface, MTFn, Factory};
use std::collections::HashMap;

use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;
use std::rc::Rc;

use generated::mediaplayer2::org_mpris_media_player2_server;
use generated::mediaplayer2_player::{org_mpris_media_player2_player_server, OrgMprisMediaPlayer2PlayerSeeked, OrgFreedesktopDBusPropertiesPropertiesChanged};

use OrgMprisMediaPlayer2Player;
use OrgMprisMediaPlayer2;

use Metadata;
use PlaybackStatus;
use LoopStatus;

pub struct MprisPlayer{
    connection: Arc<Connection>,
    factory: Arc<Factory<MTFn<TData>, TData>>,

    // OrgMprisMediaPlayer2         Type
    can_quit: Cell<bool>,           // R
    fullscreen: Cell<bool>,         // R/W
    can_set_fullscreen: Cell<bool>, // R
    can_raise: Cell<bool>,          // R
    has_track_list: Cell<bool>,     // R
    identify: String,               // R
    desktop_entry: String,          // R
    supported_uri_schemes: RefCell<Vec<String>>, // R
    supported_mime_types: RefCell<Vec<String>>,  // R

    // OrgMprisMediaPlayer2Player   Type
    playback_status: Cell<PlaybackStatus>, // R
    loop_status: Cell<LoopStatus>,  // R/W
    rate: Cell<f64>,                // R/W
    shuffle: Cell<bool>,            // R/W
    metadata: RefCell<Metadata>,    // R
    volume: Cell<f64>,              // R/W
    position: Cell<i64>,            // R
    minimum_rate: Cell<f64>,        // R
    maximum_rate: Cell<f64>,        // R
    can_go_next: Cell<bool>,        // R
    can_go_previous: Cell<bool>,    // R
    can_play: Cell<bool>,           // R
    can_pause: Cell<bool>,          // R
    can_seek: Cell<bool>,           // R
    can_control: Cell<bool>,        // R
}

impl MprisPlayer{
    pub fn new(mpris_name: String, identify: String, desktop_entry: String) -> Arc<Self>{
        let connection = Arc::new(Connection::get_private(BusType::Session).unwrap());
        let factory = Arc::new(Factory::new_fn());

        let mpris_player = Arc::new(MprisPlayer{
            connection,
            factory,

            can_quit: Cell::new(false),
            fullscreen: Cell::new(false),
            can_set_fullscreen: Cell::new(false),
            can_raise: Cell::new(false),
            has_track_list: Cell::new(false),
            identify,
            desktop_entry,
            supported_uri_schemes: RefCell::new(Vec::new()),
            supported_mime_types: RefCell::new(Vec::new()),

            playback_status: Cell::new(PlaybackStatus::Paused),
            loop_status: Cell::new(LoopStatus::None),
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

        // Create OrgMprisMediaPlayer2 interface
        let root_iface: Interface<MTFn<TData>, TData> = org_mpris_media_player2_server(&mpris_player.factory, (), |m| {
            let a: &Arc<MprisPlayer> = m.path.get_data();
            let b: &MprisPlayer = &a;
            b
        });

        // Create OrgMprisMediaPlayer2Player interface
        let player_iface: Interface<MTFn<TData>, TData> = org_mpris_media_player2_player_server(&mpris_player.factory, (), |m| {
            let a: &Arc<MprisPlayer> = m.path.get_data();
            let b: &MprisPlayer = &a;
            b
        });

        // Create dbus tree
        let mut tree = mpris_player.factory.tree(());
        tree = tree.add(mpris_player.factory.object_path("/org/mpris/MediaPlayer2", mpris_player.clone())
            .introspectable()
            .add(root_iface)
            .add(player_iface)
        );

        // Setup dbus connection
        mpris_player.connection.register_name(&format!("org.mpris.MediaPlayer2.{}", mpris_name), 0).unwrap();
        tree.set_registered(&mpris_player.connection, true).unwrap();
        mpris_player.connection.add_handler(tree);

        let connection = mpris_player.connection.clone();
        gtk::timeout_add(500, move||{
            connection.incoming(1000).next();
            glib::Continue(true)
        });

        mpris_player
    }

    pub fn property_changed<T: 'static>(&self, name: String, value: T) where T: dbus::arg::RefArg {
        let mut changed_properties = HashMap::new();
        let x = Box::new(value) as Box<RefArg>;
        changed_properties.insert(name, Variant(x));

        let signal = OrgFreedesktopDBusPropertiesPropertiesChanged {
            changed_properties,
            interface_name: "org.mpris.MediaPlayer2.Player".to_string(),
            invalidated_properties: Vec::new(),
        };

        self.connection.send(signal.to_emit_message(&Path::new("/org/mpris/MediaPlayer2").unwrap())).unwrap();
    }

    //
    // OrgMprisMediaPlayer2 setters...
    //

    pub fn set_supported_mime_types(&self, value: Vec<String>){
        *self.supported_mime_types.borrow_mut() = value;
        self.property_changed("SupportedMimeTypes".to_string(), self.get_supported_mime_types().unwrap());
    }

    pub fn set_supported_uri_schemes(&self, value: Vec<String>){
        *self.supported_uri_schemes.borrow_mut() = value;
        self.property_changed("SupportedUriSchemes".to_string(), self.get_supported_uri_schemes().unwrap());
    }

    pub fn set_can_quit(&self, value: bool){
        self.can_quit.set(value);
        self.property_changed("CanQuit".to_string(), self.get_can_quit().unwrap());
    }

    pub fn set_can_raise(&self, value: bool){
        self.can_raise.set(value);
        self.property_changed("CanRaise".to_string(), self.get_can_raise().unwrap());
    }

    pub fn set_can_set_fullscreen(&self, value: bool){
        self.can_set_fullscreen.set(value);
        self.property_changed("CanSetFullscreen".to_string(), self.get_can_set_fullscreen().unwrap());
    }

    pub fn set_has_track_list(&self, value: bool){
        self.has_track_list.set(value);
        self.property_changed("HasTrackList".to_string(), self.get_has_track_list().unwrap());
    }

    //
    // OrgMprisMediaPlayer2Player setters...
    //

    pub fn set_playback_status(&self, value: PlaybackStatus){
        self.playback_status.set(value);
        self.property_changed("PlaybackStatus".to_string(), self.get_playback_status().unwrap());
    }

    pub fn set_loop_status(&self, value: LoopStatus){
        self.loop_status.set(value);
        self.property_changed("LoopStatus".to_string(), self.get_loop_status().unwrap());
    }

    pub fn set_metadata(&self, metadata: Metadata){
        *self.metadata.borrow_mut() = metadata;
        self.property_changed("Metadata".to_string(), self.get_metadata().unwrap());
    }

    pub fn set_position(&self, value: i64){
        self.position.set(value);
        self.property_changed("Position".to_string(), self.get_position().unwrap());
    }

    pub fn set_minimum_rate(&self, value: f64){
        self.minimum_rate.set(value);
        self.property_changed("MinimumRate".to_string(), self.get_minimum_rate().unwrap());
    }

    pub fn set_maximum_rate(&self, value: f64){
        self.maximum_rate.set(value);
        self.property_changed("MaximumRate".to_string(), self.get_maximum_rate().unwrap());
    }

    pub fn set_can_go_next(&self, value: bool){
        self.can_go_next.set(value);
        self.property_changed("CanGoNext".to_string(), self.get_can_go_next().unwrap());
    }

    pub fn set_can_go_previous(&self, value: bool){
        self.can_go_previous.set(value);
        self.property_changed("CanPrevious".to_string(), self.get_can_go_previous().unwrap());
    }

    pub fn set_can_play(&self, value: bool){
        self.can_play.set(value);
        self.property_changed("CanPlay".to_string(), self.get_can_play().unwrap());
    }

    pub fn set_can_pause(&self, value: bool){
        self.can_pause.set(value);
        self.property_changed("CanPause".to_string(), self.get_can_pause().unwrap());
    }

    pub fn set_can_seek(&self, value: bool){
        self.can_seek.set(value);
        self.property_changed("CanSeek".to_string(), self.get_can_seek().unwrap());
    }

    pub fn set_can_control(&self, value: bool){
        self.can_control.set(value);
        self.property_changed("CanControl".to_string(), self.get_can_control().unwrap());
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
        self.property_changed("Fullscreen".to_string(), self.get_fullscreen().unwrap());
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
        Ok(self.supported_uri_schemes.borrow().to_vec())
    }

    fn get_supported_mime_types(&self) -> Result<Vec<String>, Self::Err> {
        Ok(self.supported_mime_types.borrow().to_vec())
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
        self.property_changed("Position".to_string(), self.get_position().unwrap());
        Ok(())
    }

    fn open_uri(&self, uri: &str) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_playback_status(&self) -> Result<String, Self::Err> {
        Ok(self.playback_status.get().value())
    }

    fn get_loop_status(&self) -> Result<String, Self::Err> {
        Ok(self.loop_status.get().value())
    }

    fn set_loop_status(&self, value: String) -> Result<(), Self::Err> {
        match value.as_ref() {
            "Track" => self.loop_status.set(LoopStatus::Track),
            "Playlist" => self.loop_status.set(LoopStatus::Playlist),
            _ => self.loop_status.set(LoopStatus::None),
        }
        self.property_changed("LoopStatus".to_string(), self.get_loop_status().unwrap());
        Ok(())
    }

    fn get_rate(&self) -> Result<f64, Self::Err> {
        Ok(self.rate.get())
    }

    fn set_rate(&self, value: f64) -> Result<(), Self::Err> {
        self.rate.set(value);
        self.property_changed("Rate".to_string(), self.get_rate().unwrap());
        Ok(())
    }

    fn get_shuffle(&self) -> Result<bool, Self::Err> {
        Ok(self.shuffle.get())
    }

    fn set_shuffle(&self, value: bool) -> Result<(), Self::Err> {
        self.shuffle.set(value);
        self.property_changed("Shuffle".to_string(), self.get_volume().unwrap());
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
        self.property_changed("Volume".to_string(), self.get_volume().unwrap());
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
