use super::data::{AudioData, ContentData, VideoData};
use libloading::Library;
use libretro_sys::{CoreAPI, LogLevel, LogPrintfFn, PixelFormat};
use once_cell::sync::Lazy;
use std::{ffi::CStr, sync::Mutex};
use tracing::{debug, info};

pub static LIBRARY: Lazy<Mutex<Option<Library>>> = Lazy::new(|| Mutex::new(None));
pub static CORE_API: Lazy<Mutex<Option<CoreAPI>>> = Lazy::new(|| Mutex::new(None));
pub static PIXEL_FORMAT: Lazy<Mutex<Option<PixelFormat>>> = Lazy::new(|| Mutex::new(None));
pub static CONTENT_DATA: Lazy<Mutex<Option<ContentData>>> = Lazy::new(|| Mutex::new(None));
pub static VIDEO_DATA: Lazy<Mutex<Option<VideoData>>> = Lazy::new(|| Mutex::new(None));
pub static AUDIO_DATA: Lazy<Mutex<Option<AudioData>>> = Lazy::new(|| Mutex::new(None));

pub unsafe extern "C" fn set_environment(command: u32, data: *mut libc::c_void) -> bool {
    match command {
        libretro_sys::ENVIRONMENT_GET_CAN_DUPE => {
            *(data as *mut bool) = true;
            false
        }
        libretro_sys::ENVIRONMENT_SET_PIXEL_FORMAT => {
            let pixel_format = PixelFormat::from_uint(*(data as *const u32)).unwrap();
            info!("Using pixel format {:#?}.", pixel_format);
            PIXEL_FORMAT.lock().unwrap().replace(pixel_format);
            true
        }
        libretro_sys::ENVIRONMENT_GET_LOG_INTERFACE => {
            *(data as *mut LogPrintfFn) = log_print;
            true
        }
        _ => false,
    }
}

unsafe extern "C" fn log_print(level: LogLevel, message: *const libc::c_char) {
    let message = CStr::from_ptr(message).to_str().unwrap();
    match level {
        LogLevel::Debug => debug!("Core debug message: {}", message),
        LogLevel::Info => debug!("Core info message: {}", message),
        LogLevel::Warn => debug!("Core warn message: {}", message),
        LogLevel::Error => debug!("Core error message: {}", message),
    }
}
