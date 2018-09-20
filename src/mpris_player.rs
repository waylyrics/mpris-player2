extern crate dbus;
use dbus::tree;
use dbus::arg::{Variant, RefArg};
use std::collections::HashMap;

use mediaplayer2_player::OrgMprisMediaPlayer2Player;
use mediaplayer2::OrgMprisMediaPlayer2;


#[derive(Debug)]
pub struct MprisPlayer{

}

impl MprisPlayer{
    pub fn new() -> Self{
        MprisPlayer{
        }
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
        Ok("".to_string())
    }

    fn get_loop_status(&self) -> Result<String, Self::Err> {
        Ok("".to_string())
    }

    fn set_loop_status(&self, value: String) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_rate(&self) -> Result<f64, Self::Err> {
        Ok(0_f64)
    }

    fn set_rate(&self, value: f64) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_shuffle(&self) -> Result<bool, Self::Err> {
        Ok(false)
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
        Ok(0_f64)
    }

    fn set_volume(&self, value: f64) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_position(&self) -> Result<i64, Self::Err> {
        Ok(0)
    }

    fn get_minimum_rate(&self) -> Result<f64, Self::Err> {
        Ok(0_f64)
    }

    fn get_maximum_rate(&self) -> Result<f64, Self::Err> {
        Ok(0_f64)
    }

    fn get_can_go_next(&self) -> Result<bool, Self::Err> {
        Ok(true)
    }

    fn get_can_go_previous(&self) -> Result<bool, Self::Err> {
        Ok(true)
    }

    fn get_can_play(&self) -> Result<bool, Self::Err> {
        Ok(true)
    }

    fn get_can_pause(&self) -> Result<bool, Self::Err> {
        Ok(true)
    }

    fn get_can_seek(&self) -> Result<bool, Self::Err> {
        Ok(true)
    }

    fn get_can_control(&self) -> Result<bool, Self::Err> {
        Ok(true)
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
        Ok(false)
    }

    fn get_fullscreen(&self) -> Result<bool, Self::Err> {
        Ok(false)
    }

    fn set_fullscreen(&self, value: bool) -> Result<(), Self::Err> {
        Ok(())
    }

    fn get_can_set_fullscreen(&self) -> Result<bool, Self::Err> {
        Ok(false)
    }

    fn get_can_raise(&self) -> Result<bool, Self::Err> {
        Ok(false)
    }

    fn get_has_track_list(&self) -> Result<bool, Self::Err> {
        Ok(false)
    }

    fn get_identity(&self) -> Result<String, Self::Err> {
        Ok("".to_string())
    }

    fn get_desktop_entry(&self) -> Result<String, Self::Err> {
        Ok("".to_string())
    }

    fn get_supported_uri_schemes(&self) -> Result<Vec<String>, Self::Err> {
        Ok(Vec::new())
    }

    fn get_supported_mime_types(&self) -> Result<Vec<String>, Self::Err> {
        Ok(Vec::new())
    }
}
