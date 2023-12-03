#[derive(Debug, Clone)]
pub struct ContentData {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub sample_rate: f64,
}

impl ContentData {
    pub fn new(width: u32, height: u32, fps: f64, sample_rate: f64) -> Self {
        Self {
            width,
            height,
            fps,
            sample_rate,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VideoData {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl VideoData {
    pub fn new(width: u32, height: u32, buffer: &[u8]) -> Self {
        Self {
            width,
            height,
            buffer: buffer.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AudioData {
    pub buffer: Vec<i16>,
}

impl AudioData {
    pub fn new(buffer: &[i16]) -> Self {
        Self {
            buffer: buffer.into(),
        }
    }
}
