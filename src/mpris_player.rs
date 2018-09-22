extern crate dbus;
use dbus::tree;
use dbus::arg::{Variant, RefArg};
use std::collections::HashMap;

use mediaplayer2_player::OrgMprisMediaPlayer2Player;
use mediaplayer2::OrgMprisMediaPlayer2;


#[derive(Debug)]
pub struct MprisPlayer{
    // OrgMprisMediaPlayer2
    can_quit: bool,
    fullscreen: bool,
    can_set_fullscreen: bool,
    can_raise: bool,
    has_track_list: bool,
    identify: String,
    desktop_entry: String,
    supported_uri_schemes: Vec<String>,
    supported_mime_types: Vec<String>,

    // OrgMprisMediaPlayer2Player
    playback_status: String,
    loop_status: String,
    rate: f64,
    shuffle: bool,
    metadata: HashMap<String, Variant<Box<RefArg + 'static>>>,
    volume: f64,
    position: i64,
    minimum_rate: f64,
    maximum_rate: f64,
    can_go_next: bool,
    can_go_previous: bool,
    can_play: bool,
    can_pause: bool,
    can_seek: bool,
    can_control: bool,
}

impl MprisPlayer{
    pub fn new() -> Self{
        MprisPlayer{
            can_quit: false,
            fullscreen: false,
            can_set_fullscreen: false,
            can_raise: false,
            has_track_list: false,
            identify: "".to_string(),
            desktop_entry: "".to_string(),
            supported_uri_schemes: Vec::new(),
            supported_mime_types: Vec::new(),

            playback_status: "".to_string(),
            loop_status: "".to_string(),
            rate: 0_f64,
            shuffle: false,
            metadata: HashMap::new(),
            volume: 0_f64,
            position: 0,
            minimum_rate: 0_f64,
            maximum_rate: 0_f64,
            can_go_next: true,
            can_go_previous: true,
            can_play: true,
            can_pause: true,
            can_seek: false,
            can_control: true,
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
        Ok(self.can_quit)
    }

    fn get_fullscreen(&self) -> Result<bool, Self::Err> {
        Ok(self.fullscreen)
    }

    fn set_fullscreen(&self, value: bool) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_can_set_fullscreen(&self) -> Result<bool, Self::Err> {
        Ok(self.can_set_fullscreen)
    }

    fn get_can_raise(&self) -> Result<bool, Self::Err> {
        Ok(self.can_raise)
    }

    fn get_has_track_list(&self) -> Result<bool, Self::Err> {
        Ok(self.has_track_list)
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
        Ok(())
    }

    fn open_uri(&self, uri: &str) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_playback_status(&self) -> Result<String, Self::Err> {
        Ok(self.playback_status.clone())
    }

    fn get_loop_status(&self) -> Result<String, Self::Err> {
        Ok(self.loop_status.clone())
    }

    fn set_loop_status(&self, value: String) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_rate(&self) -> Result<f64, Self::Err> {
        Ok(self.rate)
    }

    fn set_rate(&self, value: f64) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_shuffle(&self) -> Result<bool, Self::Err> {
        Ok(self.shuffle)
    }

    fn set_shuffle(&self, value: bool) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_metadata(&self) -> Result<HashMap<String, Variant<Box<RefArg + 'static>>>, Self::Err> {
        let mut metadata = HashMap::new();

        let x = Box::new("It's working! Yay!!".to_string()) as Box<RefArg>;
        metadata.insert("xesam:title".to_string(), Variant(x));

        let x = Box::new("https://cdn.freebiesupply.com/logos/large/2x/rust-logo-png-transparent.png".to_string()) as Box<RefArg>;
        metadata.insert("mpris:artUrl".to_string(), Variant(x));

        Ok(metadata)
    }

    fn get_volume(&self) -> Result<f64, Self::Err> {
        Ok(self.volume)
    }

    fn set_volume(&self, value: f64) -> Result<(), Self::Err> {
        //self.volume = value;
        Ok(())
    }

    fn get_position(&self) -> Result<i64, Self::Err> {
        Ok(self.position)
    }

    fn get_minimum_rate(&self) -> Result<f64, Self::Err> {
        Ok(self.minimum_rate)
    }

    fn get_maximum_rate(&self) -> Result<f64, Self::Err> {
        Ok(self.maximum_rate)
    }

    fn get_can_go_next(&self) -> Result<bool, Self::Err> {
        Ok(self.can_go_next)
    }

    fn get_can_go_previous(&self) -> Result<bool, Self::Err> {
        Ok(self.can_go_previous)
    }

    fn get_can_play(&self) -> Result<bool, Self::Err> {
        Ok(self.can_play)
    }

    fn get_can_pause(&self) -> Result<bool, Self::Err> {
        Ok(self.can_pause)
    }

    fn get_can_seek(&self) -> Result<bool, Self::Err> {
        Ok(self.can_seek)
    }

    fn get_can_control(&self) -> Result<bool, Self::Err> {
        Ok(self.can_control)
    }
}
