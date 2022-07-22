extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::util::frame::video::Video;
use std::fs::File;
use std::io::prelude::*;
use log::{info};

mod wgpu;
mod video;

fn process_frame(frame: &Video, index: usize) -> Result<(), std::io::Error> {
    info!("Process frame {} - {} x {}", index, frame.width(), frame.height());
    let raw = frame.data(0);

    let mut file = File::create("frame.ppm")?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(raw)?;
    Ok(())
}

fn main()  -> Result<(), ffmpeg::Error> {
    env_logger::init();
    let file = "test/VID_20211112_112308_00_153.insv";

    let mut left = video::FrameExtractor::new(file.to_string())?;
    left.get_frame();

     //pollster::block_on(wgpu::run());
    Ok(())
}
