extern crate ffmpeg_next as ffmpeg;

use ffmpeg::util::frame::video::Video;
use std::fs::File;
use std::io::prelude::*;
use log::{info};
use core::f64::consts::PI;

mod wgpu;
mod video;

fn concatenate_frames(left: &[u8], right: &[u8], width: usize, height: usize) -> Result<(), std::io::Error> {
    info!("Concat frame - {} x {}", width, height);
    let mut file = File::create("frame.ppm")?;
    file.write_all(format!("P6\n{} {}\n255\n", width *2, height).as_bytes())?;

    let mut i = 0;
    for row in 0..height {
        file.write_all(&left[i*3..(i+width)*3]);
        file.write_all(&right[i*3..(i+width)*3]);
        i += width;
    }
    Ok(())
}


fn copy_pixel(ind_src: usize, src: &[u8], ind_dest: usize, dest: &mut [u8]) {
    dest[ind_dest*3] = src[ind_src*3];
    dest[ind_dest*3+1] = src[ind_src*3+1];
    dest[ind_dest*3+2] = src[ind_src*3+2];
}

fn naive_fisheye_reproject(data: &[u8], width: usize, height: usize) -> Vec<u8> {
    let mut out = vec![0; width * height * 3]; 
    assert!(data.len() == width * height * 3);
    for y in 0..height {
        for x in 0..width {
            let ind_dest = y*width + x;

            let lon = x as f64 / width as f64 * PI;
            let lat = y as f64 / height as f64 * PI;

            // We have lon - the angle from west along the horizontal plane
            // We have lat - the angle from north along the vertical plane
            // We want to project the vector rotated along both of these onto the third
            // plane of axis.
            //
            // thus we need the projective angle
            //
            // project a onto b is a.b
            // a1 = |a| cos theta
            // or a1 = 1 * cos theta as we use unit vector.
            

            let xs = (((lon).cos() * lat.sin() * ((width-1)/2) as f64).floor() + (width/2) as f64) as usize;
            let ys = (((lat).cos() * ((height-1)/2) as f64).floor() + (height/2) as f64) as usize; 

            //info!("xy {},{} -> lon, lat {},{} -> x,y {},{}", x, y, lon, lat, xs, ys);

            let ind_src = ys * width + xs;
            copy_pixel(ind_src,data, ind_dest, &mut out);
        }
    }

    return out;
}

fn main()  -> Result<(), ffmpeg::Error> {
    env_logger::init();

    let mut frames = video::DualFrameExtractor::new(
        "test/VID_20211112_112308_00_153.insv",
        "test/VID_20211112_112308_10_153.insv"
    )?;

    let (left, right, ind) = frames.get_frame()?;
    process_frame(&left, &right, ind);
    let l = &naive_fisheye_reproject(left.data(0), left.width() as usize, left.height() as usize);
    let r = &naive_fisheye_reproject(right.data(0), right.width() as usize, right.height() as usize);
    concatenate_frames(r, l, left.width() as usize, left.height() as usize);

     //pollster::block_on(wgpu::run());
    Ok(())
}
