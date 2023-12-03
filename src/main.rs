use ggez::{
    conf::{WindowMode, WindowSetup},
    event::EventHandler,
    glam::Vec2,
    graphics::{Canvas, Color, DrawParam, Image, ImageFormat, Sampler},
    winit::dpi::PhysicalSize,
    Context, ContextBuilder, GameError, GameResult,
};
use libretro_frontend::retro::{self, ContentData};
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};
use std::{sync::mpsc::Sender, thread::JoinHandle};
use tracing::Level;

struct MainState {
    content_data: ContentData,
    audio_sender: Sender<Vec<i16>>,
    audio_thread: Option<JoinHandle<()>>,
}

impl MainState {
    fn new(_context: &mut Context) -> GameResult<Self> {
        unsafe {
            retro::load("core")
                .map_err(|_| GameError::FilesystemError("Core file not found.".to_owned()))?
        };
        unsafe {
            retro::start("content")
                .map_err(|_| GameError::FilesystemError("Content file not found.".to_owned()))?
        };

        let content_data = retro::get_content_data().unwrap();

        let (audio_sender, audio_receiver): (Sender<Vec<i16>>, _) = std::sync::mpsc::channel();
        let audio_thread = Some(std::thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            loop {
                let audio_buffer: Vec<i16> = audio_receiver.recv().unwrap();
                let sample = SamplesBuffer::new(2, content_data.sample_rate as u32, audio_buffer);
                sink.append(sample);
            }
        }));

        Ok(Self {
            content_data,
            audio_sender,
            audio_thread,
        })
    }

    fn calc_dest_scale(
        ideal_size: PhysicalSize<u32>,
        buffer_size: PhysicalSize<u32>,
        window_dimensions: PhysicalSize<u32>,
    ) -> (Vec2, Vec2) {
        let scale_aspect_x = ideal_size.width as f32 / buffer_size.width as f32;
        let scale_aspect_y = ideal_size.height as f32 / buffer_size.height as f32;

        let scale_x = window_dimensions.width as f32 / ideal_size.width as f32;
        let scale_y = window_dimensions.height as f32 / ideal_size.height as f32;
        let scale_factor = scale_x.min(scale_y);

        let dest = Vec2::new(
            (window_dimensions.width as f32 - ideal_size.width as f32 * scale_factor) / 2.0,
            (window_dimensions.height as f32 - ideal_size.height as f32 * scale_factor) / 2.0,
        );

        let scale = Vec2::new(scale_aspect_x * scale_factor, scale_aspect_y * scale_factor);
        (dest, scale)
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult {
        while context.time.check_update_time(self.content_data.fps as u32) {
            unsafe { retro::update() };
            if let Some(audio_data) = retro::get_audio_data() {
                self.audio_sender.send(audio_data.buffer).unwrap();
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        if let Some(video_buffer) = retro::get_video_data() {
            let mut canvas = Canvas::from_frame(context, Color::BLACK);
            canvas.set_sampler(Sampler::nearest_clamp());

            let ideal_size = PhysicalSize::new(self.content_data.width, self.content_data.height);
            let buffer_size = PhysicalSize::new(video_buffer.width, video_buffer.height);
            let window_dimensions = context.gfx.window().inner_size();

            let image = Image::from_pixels(
                context,
                &video_buffer.buffer,
                ImageFormat::Rgba8UnormSrgb,
                video_buffer.width,
                video_buffer.height,
            );

            let (dest, scale) = Self::calc_dest_scale(ideal_size, buffer_size, window_dimensions);

            canvas.draw(&image, DrawParam::new().scale(scale).dest(dest));
            canvas.finish(context)?;
        }

        Ok(())
    }

    fn quit_event(&mut self, _context: &mut Context) -> GameResult<bool> {
        unsafe { retro::deinit() };
        self.audio_thread.take();
        Ok(false)
    }
}

pub fn main() -> GameResult {
    tracing_subscriber::fmt().with_max_level(Level::WARN).init();

    let context_builder = ContextBuilder::new("libretro-frontend", "ggez")
        .window_mode(
            WindowMode::default()
                .resizable(true)
                .dimensions(640.0, 576.0),
        )
        .window_setup(
            WindowSetup::default()
                .title("Libretro Frontend")
                .vsync(true),
        );

    let (mut context, event_loop) = context_builder.build()?;
    let state = MainState::new(&mut context)?;

    ggez::event::run(context, event_loop, state)
}
