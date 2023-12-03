use super::{data::AudioData, state};

pub unsafe extern "C" fn set_audio_sample(_left: i16, _right: i16) {}

pub unsafe extern "C" fn set_audio_sample_batch(
    data: *const i16,
    frames: libc::size_t,
) -> libc::size_t {
    let data = std::slice::from_raw_parts(data, frames * 2);
    state::AUDIO_DATA
        .lock()
        .unwrap()
        .replace(AudioData::new(data));
    frames
}
