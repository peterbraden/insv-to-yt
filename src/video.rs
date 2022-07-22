
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::fs::File;
use std::io::prelude::*;
use log::{info};

pub struct FrameExtractor {
    decoder: ffmpeg::decoder::Video,
    scaler: ffmpeg::software::scaling::context::Context,
    frame_index: usize,
}
/**
 * Patch over the horrible ffmpeg video API to allow us to grab a frame from a video
 */
impl FrameExtractor {
    pub fn new(filename: String) -> Result<Self, ffmpeg::Error> {
        let mut ictx = ffmpeg::format::input(&filename)?;
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

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                info!("Send packet");
                loop {
                    match decoder.send_packet(&packet) {
                        Ok(()) => {break;},
                        Err(e) => {
                            if e == (ffmpeg::Error::Other { errno: ffmpeg::util::error::EAGAIN }) {
                                info!("EAGAIN");
                                continue
                            }
                            return Err(e);
                            info!("ERR {:?}", e);
                        }
                    };
                }
            }
        }
        decoder.send_eof()?;

        Ok(
            FrameExtractor {
                decoder,
                scaler,
                frame_index
            }
        )
     }

    pub fn get_frame(&mut self) -> Result<Video, ffmpeg::Error> {
        info!("Get frame {}", self.frame_index);
        let mut decoded = Video::empty();
        self.decoder.receive_frame(&mut decoded)?;
        let mut rgb_frame = Video::empty();
        self.scaler.run(&decoded, &mut rgb_frame)?;
        self.frame_index += 1;
        return Ok(rgb_frame);
    }
}


