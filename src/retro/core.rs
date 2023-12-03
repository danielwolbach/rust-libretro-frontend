use super::{error::*, *};
use crate::retro::data::ContentData;
use libloading::Library;
use libretro_sys::{CoreAPI, GameGeometry, GameInfo, SystemAvInfo, SystemTiming};
use std::{
    ffi::{CString, OsStr},
    fmt::Debug,
    ops::Deref,
    path::Path,
};
use tracing::info;

macro_rules! library_get {
    ($library:expr, $function_name:expr) => {
        *($library
            .get($function_name.as_bytes())
            .map_err(Error::Api)?)
    };
}

/// Load a Libretro Core from the specified file path and initializes the Core
/// API.
///
/// # Safety
///
/// This function is marked as `unsafe` as it involves loading and interacting
/// with a dynamic library. It assumes that the dynamic library at the specified
/// path is a valid Libretro Core.
///
/// # Arguments
///
/// * `path` - The file path of the Libretro Core dynamic library to be loaded.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the loading process. An
/// `Err` variant is returned in case of loading errors, with an associated
/// `Error` enum.
pub unsafe fn load<P: AsRef<OsStr> + Debug>(path: P) -> Result<()> {
    let library = Library::new(&path).map_err(Error::Library)?;
    let core_api = CoreAPI {
        retro_set_environment: library_get!(library, "retro_set_environment"),
        retro_set_video_refresh: library_get!(library, "retro_set_video_refresh"),
        retro_set_audio_sample: library_get!(library, "retro_set_audio_sample"),
        retro_set_audio_sample_batch: library_get!(library, "retro_set_audio_sample_batch"),
        retro_set_input_poll: library_get!(library, "retro_set_input_poll"),
        retro_set_input_state: library_get!(library, "retro_set_input_state"),
        retro_init: library_get!(library, "retro_init"),
        retro_deinit: library_get!(library, "retro_deinit"),
        retro_api_version: library_get!(library, "retro_api_version"),
        retro_get_system_info: library_get!(library, "retro_get_system_info"),
        retro_get_system_av_info: library_get!(library, "retro_get_system_av_info"),
        retro_set_controller_port_device: library_get!(library, "retro_set_controller_port_device"),
        retro_reset: library_get!(library, "retro_reset"),
        retro_run: library_get!(library, "retro_run"),
        retro_serialize_size: library_get!(library, "retro_serialize_size"),
        retro_serialize: library_get!(library, "retro_serialize"),
        retro_unserialize: library_get!(library, "retro_unserialize"),
        retro_cheat_reset: library_get!(library, "retro_cheat_reset"),
        retro_cheat_set: library_get!(library, "retro_cheat_set"),
        retro_load_game: library_get!(library, "retro_load_game"),
        retro_load_game_special: library_get!(library, "retro_load_game_special"),
        retro_unload_game: library_get!(library, "retro_unload_game"),
        retro_get_region: library_get!(library, "retro_get_region"),
        retro_get_memory_data: library_get!(library, "retro_get_memory_data"),
        retro_get_memory_size: library_get!(library, "retro_get_memory_size"),
    };

    info!("Loaded Core from path {:#?}.", path);

    state::LIBRARY.lock().unwrap().replace(library);
    state::CORE_API.lock().unwrap().replace(core_api);

    Ok(())
}

/// Initializes and starts a Libretro core with the specified content file.
///
/// # Safety
///
/// This function is marked as `unsafe` due to its interactions with the
/// Libretro Core API and assumes correct usage by the caller.
///
/// # Arguments
///
/// * `path` - The file path of the content to be loaded by the Libretro Core.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the operation. An `Err`
/// variant is returned in case of content errors, with an associated `Error`
/// enum.
pub unsafe fn start<P: AsRef<Path> + Debug + Into<Vec<u8>>>(path: P) -> Result<()> {
    if let Some(core_api) = state::CORE_API.lock().unwrap().deref() {
        // Configure Libretro Core API callbacks.
        (core_api.retro_set_environment)(state::set_environment);
        (core_api.retro_init)();
        (core_api.retro_set_video_refresh)(video::set_video_refresh);
        (core_api.retro_set_input_poll)(input::set_input_poll);
        (core_api.retro_set_input_state)(input::set_input_state);
        (core_api.retro_set_audio_sample)(audio::set_audio_sample);
        (core_api.retro_set_audio_sample_batch)(audio::set_audio_sample_batch);

        info!("Initialized Core callbacks.");

        // Load the content.
        let contents = std::fs::read(&path).map_err(Error::Content)?;
        let path = CString::new(path).unwrap();
        let game_info = GameInfo {
            path: path.as_ptr(),
            data: contents.as_ptr() as *const libc::c_void,
            size: contents.len(),
            meta: std::ptr::null(),
        };
        (core_api.retro_load_game)(&game_info);

        info!("Loaded content from path {:#?}.", path);

        // Load the dcontent data.
        let mut system_av_info = SystemAvInfo {
            geometry: GameGeometry {
                base_width: 0,
                base_height: 0,
                max_width: 0,
                max_height: 0,
                aspect_ratio: 0.0,
            },
            timing: SystemTiming {
                fps: 0.0,
                sample_rate: 0.0,
            },
        };
        (core_api.retro_get_system_av_info)(&mut system_av_info);
        let content_data = ContentData::new(
            system_av_info.geometry.base_width,
            system_av_info.geometry.base_height,
            system_av_info.timing.fps,
            system_av_info.timing.sample_rate,
        );
        info!("Found content info: {:?}", content_data);
        state::CONTENT_DATA.lock().unwrap().replace(content_data);
    }

    Ok(())
}

/// Update the Libretro Core for keeping the content running.
///
/// # Safety
///
/// This function is marked as `unsafe` due to its interactions with the
/// Libretro Core API and assumes correct usage by the caller.
pub unsafe fn update() {
    if let Some(core_api) = state::CORE_API.lock().unwrap().deref() {
        (core_api.retro_run)();
    }
}

/// De-initialize the Libretro Core and reset all states.
///
/// # Safety
///
/// This function is marked as `unsafe` due to its interactions with the
/// Libretro Core API and assumes correct usage by the caller.
pub unsafe fn deinit() {
    if let Some(core_api) = state::CORE_API.lock().unwrap().deref() {
        (core_api.retro_deinit)();
    }

    state::LIBRARY.lock().unwrap().take();
    state::CORE_API.lock().unwrap().take();
    state::PIXEL_FORMAT.lock().unwrap().take();
    state::VIDEO_DATA.lock().unwrap().take();
}
