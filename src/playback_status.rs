#[derive(Debug)]
pub enum PlaybackStatus{
    Playing,
    Paused,
    Stopped,
}

impl PlaybackStatus {
    pub fn value(&self) -> String {
        match *self {
            PlaybackStatus::Playing => "Playing".to_string(),
            PlaybackStatus::Paused => "Paused".to_string(),
            PlaybackStatus::Stopped => "Stopped".to_string(),
        }
    }
}
