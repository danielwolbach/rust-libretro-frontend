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

/// Retrieves the content data, if available.
///
/// This function attempts to acquire a lock on the global CONTENT_DATA state
/// and returns an `Option<ContentData>` representing the content data if
/// the lock is successfully acquired.
///
/// # Returns
/// - `Some(ContentData)`: If the lock is successfully acquired, returns a clone
///   of the content data.
/// - `None`: If the lock cannot be acquired or if the content data is not available.
///
/// # Examples
/// ```
/// # use your_crate_name::get_content_data;
/// // Attempt to retrieve content data.
/// if let Some(content_data) = get_content_data() {
///     // Process the content data.
///     println!("Content Title: {}", content_data.title);
/// } else {
///     // Handle the case when content data is not available.
///     println!("Content data is not available.");
/// }
/// ```
pub fn get_content_data() -> Option<ContentData> {
    state::CONTENT_DATA.lock().ok()?.clone()
}

pub fn get_video_data() -> Option<VideoData> {
    state::VIDEO_DATA.lock().ok()?.clone()
}

pub fn get_audio_data() -> Option<AudioData> {
    state::AUDIO_DATA.lock().ok()?.clone()
}
