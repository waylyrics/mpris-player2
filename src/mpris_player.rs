extern crate dbus;
use dbus::tree;
use dbus::arg::{Variant, RefArg};
use std::collections::HashMap;
use std::cell::Cell;

use mediaplayer2_player::OrgMprisMediaPlayer2Player;
use mediaplayer2::OrgMprisMediaPlayer2;


pub struct MprisPlayer{
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
    metadata: Cell<Metadata>,
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
    pub fn new() -> Self{
        MprisPlayer{
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
            metadata: Cell::new(Metadata::new()),
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
        //let metadata = self.metadata.borrow().clone().to_hashmap();
        //Ok(metadata)
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

#[derive(Debug, Clone)]
pub struct Metadata{
    pub length: i64,
    pub art_url: String,
    pub album: String,
    pub album_artist: Vec<String>,
    pub artist: Vec<String>,
    pub composer: Vec<String>,
    pub disc_number: i32,
    pub genre: Vec<String>,
    pub title: String,
    pub track_number: i32,
    pub url: String,
}

impl Metadata{
    pub fn new() -> Self{
        Metadata{
            length: 0,
            art_url: "".to_string(),
            album: "".to_string(),
            album_artist: Vec::new(),
            artist: Vec::new(),
            composer: Vec::new(),
            disc_number: 0,
            genre: Vec::new(),
            title: "".to_string(),
            track_number: 0,
            url: "".to_string(),
        }
    }

    pub fn to_hashmap(&self) -> HashMap<String, Variant<Box<RefArg + 'static>>> {
        let mut metadata = HashMap::new();

        let x = Box::new(self.length.to_string()) as Box<RefArg>;
        metadata.insert("mpris:length".to_string(), Variant(x));

        let x = Box::new(self.art_url.clone()) as Box<RefArg>;
        metadata.insert("mpris:artUrl".to_string(), Variant(x));

        let x = Box::new(self.album.clone()) as Box<RefArg>;
        metadata.insert("xesam:album".to_string(), Variant(x));

        let x = Box::new(self.album_artist.clone()) as Box<RefArg>;
        metadata.insert("xesam:albumArtist".to_string(), Variant(x));

        let x = Box::new(self.artist.clone()) as Box<RefArg>;
        metadata.insert("xesam:artist".to_string(), Variant(x));

        let x = Box::new(self.composer.clone()) as Box<RefArg>;
        metadata.insert("xesam:composer".to_string(), Variant(x));

        let x = Box::new(self.disc_number) as Box<RefArg>;
        metadata.insert("xesam:discNumber".to_string(), Variant(x));

        let x = Box::new(self.genre.clone()) as Box<RefArg>;
        metadata.insert("xesam:genre".to_string(), Variant(x));

        let x = Box::new(self.title.clone()) as Box<RefArg>;
        metadata.insert("xesam:title".to_string(), Variant(x));

        let x = Box::new(self.track_number) as Box<RefArg>;
        metadata.insert("xesam:trackNumber".to_string(), Variant(x));

        let x = Box::new(self.url.clone()) as Box<RefArg>;
        metadata.insert("xesam:url".to_string(), Variant(x));

        metadata
    }
}


#[derive(Debug)]
pub enum PlaybackStatus{
    Playing,
    Paused,
    Stopped,
}

impl PlaybackStatus {
    fn value(&self) -> String {
        match *self {
            PlaybackStatus::Playing => "Playing".to_string(),
            PlaybackStatus::Paused => "Paused".to_string(),
            PlaybackStatus::Stopped => "Stopped".to_string(),
        }
    }
}
