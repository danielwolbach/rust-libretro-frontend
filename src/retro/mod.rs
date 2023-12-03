mod audio;
mod core;
mod data;
mod error;
mod input;
mod state;
mod video;

pub use core::deinit;
pub use core::load;
pub use core::start;
pub use core::update;
pub use data::AudioData;
pub use data::ContentData;
pub use data::VideoData;

pub fn get_content_data() -> Option<ContentData> {
    state::CONTENT_DATA.lock().ok()?.clone()
}

pub fn get_video_data() -> Option<VideoData> {
    state::VIDEO_DATA.lock().ok()?.clone()
}

pub fn get_audio_data() -> Option<AudioData> {
    state::AUDIO_DATA.lock().ok()?.clone()
}
