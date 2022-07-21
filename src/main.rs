extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use log::{info, warn};

mod wgpu;

fn process_frame(frame: &Video, index: usize) -> Result<(), std::io::Error> {
    info!("Process frame {} - {} x {}", index, frame.width(), frame.height());
    let raw = frame.data(0);

    let mut file = File::create("frame.ppm")?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(raw)?;
    Ok(())
}

fn extract_frames(filename: String) -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();
    let mut ictx = input(&filename)?;
    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut frame_index = 0;
    let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    process_frame(&rgb_frame, frame_index);
                    frame_index += 1;
                }
                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                receive_and_process_decoded_frames(&mut decoder)?;
            }
        }
        decoder.send_eof()?;
        receive_and_process_decoded_frames(&mut decoder)?;
        Ok(())
}


fn main()  -> Result<(), ffmpeg::Error> {
    env_logger::init();
    let file = "test/VID_20211112_112308_00_153.insv";
    extract_frames(file.to_string())?;

     //pollster::block_on(wgpu::run());
    Ok(())
}
