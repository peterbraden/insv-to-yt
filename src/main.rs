extern crate ffmpeg_next as ffmpeg;

use ffmpeg::util::frame::video::Video;
use std::fs::File;
use std::io::prelude::*;
use log::{info};

mod wgpu;
mod video;

fn process_frame(left: &Video, right: &Video, index: usize) -> Result<(), std::io::Error> {
    info!("Process frame {} - {} x {}", index, left.width(), left.height());
    let mut file = File::create("frame.ppm")?;
    file.write_all(format!("P6\n{} {}\n255\n", left.width(), left.height() + right.height()).as_bytes())?;
    file.write_all(left.data(0))?;
    file.write_all(right.data(0))?;
    Ok(())
}

fn main()  -> Result<(), ffmpeg::Error> {
    env_logger::init();


    let mut frames = video::DualFrameExtractor::new(
        "test/VID_20211112_112308_00_153.insv",
        "test/VID_20211112_112308_10_153.insv"
    )?;

    let (left, right, ind) = frames.get_frame()?;
    process_frame(&left, &right, ind);

     //pollster::block_on(wgpu::run());
    Ok(())
}
