use super::{data::VideoData, state};
use libretro_sys::PixelFormat;

pub unsafe extern "C" fn set_video_refresh(
    data: *const libc::c_void,
    width: libc::c_uint,
    height: libc::c_uint,
    pitch: libc::size_t,
) {
    if data.is_null() {
        return;
    }

    let pixel_format = *state::PIXEL_FORMAT.lock().unwrap();

    let bytes_per_pixel = match pixel_format {
        Some(PixelFormat::RGB565) => 2,
        Some(PixelFormat::ARGB1555) => 2,
        Some(PixelFormat::ARGB8888) => 4,
        _ => panic!(),
    };

    let data = std::slice::from_raw_parts(data as *const u8, pitch * height as usize);
    let data = remove_pitch(data, width, pitch, bytes_per_pixel);
    let data = match pixel_format {
        Some(PixelFormat::RGB565) => rgb565_to_rgb8888(&data),
        Some(PixelFormat::ARGB1555) => todo!(),
        Some(PixelFormat::ARGB8888) => rgb8888_to_rgb8888(&data),
        _ => panic!(),
    };

    let video_buffer = VideoData::new(width, height, &data);
    state::VIDEO_DATA.lock().unwrap().replace(video_buffer);
}

pub fn remove_pitch(data: &[u8], width: u32, pitch: usize, bytes_per_pixel: usize) -> Vec<u8> {
    if data.len() % pitch != 0 {
        panic!("Image data size is not a multiple of pitch.");
    }

    let mut result = Vec::with_capacity(data.len());

    for row in data.chunks_exact(pitch) {
        for &value in row.iter().take(width as usize * bytes_per_pixel) {
            result.push(value);
        }
    }

    result
}

pub fn rgb565_to_rgb8888(data: &[u8]) -> Vec<u8> {
    if data.len() % 2 != 0 {
        panic!("RGB565 image data size is not a multiple of 2.");
    }

    let mut result = Vec::with_capacity(data.len() / 2 * 4);

    for i in (0..data.len()).step_by(2) {
        let lo = data[i];
        let hi = data[i + 1];

        let r = (hi & 0b1111_1000) >> 3;
        let g = ((hi & 0b0000_0111) << 3) + ((lo & 0b1110_0000) >> 5);
        let b = lo & 0b0001_1111;
        let r = (r << 3) | (r >> 2);
        let g = (g << 2) | (g >> 3);
        let b = (b << 3) | (b >> 2);

        result.extend_from_slice(&[r, g, b, 255u8]);
    }

    result
}

pub fn rgb8888_to_rgb8888(data: &[u8]) -> Vec<u8> {
    if data.len() % 4 != 0 {
        panic!("RGB8888 image data size is not a multiple of 3.");
    }

    let mut result = Vec::with_capacity(data.len());

    for i in (0..data.len()).step_by(4) {
        let r = data[i + 2];
        let g = data[i + 1];
        let b = data[i];

        result.extend_from_slice(&[r, g, b, 255u8]);
    }

    result
}
