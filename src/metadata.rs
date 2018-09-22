use dbus::arg::{Variant, RefArg};
use std::collections::HashMap;

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
