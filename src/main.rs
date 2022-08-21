use std::fs::File;

use anyhow::Result;
use webm::mux::{Segment, Track};

fn main() -> Result<()> {
    let file = File::create("test.webm")?;
    let muxer = webm::mux::Writer::new(file);
    let mut segment = Segment::new(muxer).ok_or(anyhow::anyhow!("Failed creating segment"))?;
    let mut track = segment.add_video_track(1280, 2560, None, webm::mux::VideoCodecId::VP8);
    let mut encoder = vpx_encode::Encoder::new(vpx_encode::Config {
        width: 1280,
        height: 2560,
        timebase: [1, 1],
        bitrate: 1000,
        codec: vpx_encode::VideoCodecId::VP8,
    })?;
    for frame in encoder.encode(0, &[0u8; 9830400])? {
        track.add_frame(frame.data, (frame.pts * 1_000_000_000) as u64, frame.key);
    }
    for frame in encoder.encode(1, &[0u8; 9830400])? {
        track.add_frame(frame.data, (frame.pts * 1_000_000_000) as u64, frame.key);
    }
    let mut finish = encoder.finish()?;
    while let Some(frame) = finish.next()? {
        track.add_frame(frame.data, (frame.pts * 1_000_000_000) as u64, frame.key);
    }
    let mut encoder = vpx_encode::Encoder::new(vpx_encode::Config {
        width: 1280,
        height: 1280,
        timebase: [1, 1],
        bitrate: 1000,
        codec: vpx_encode::VideoCodecId::VP8,
    })?;
    for frame in encoder.encode(2, &[127u8; 4915200])? {
        track.add_frame(frame.data, (frame.pts * 1_000_000_000) as u64, frame.key);
    }
    for frame in encoder.encode(3, &[127u8; 4915200])? {
        track.add_frame(frame.data, (frame.pts * 1_000_000_000) as u64, frame.key);
    }
    let mut finish = encoder.finish()?;
    while let Some(frame) = finish.next()? {
        track.add_frame(frame.data, (frame.pts * 1_000_000_000) as u64, frame.key);
    }
    let mut encoder = vpx_encode::Encoder::new(vpx_encode::Config {
        width: 320,
        height: 640,
        timebase: [1, 1],
        bitrate: 1000,
        codec: vpx_encode::VideoCodecId::VP8,
    })?;
    for frame in encoder.encode(4, &[255u8; 614400])? {
        track.add_frame(frame.data, (frame.pts * 1_000_000_000) as u64, frame.key);
    }
    for frame in encoder.encode(i64::MAX, &[255u8; 614400])? {
        track.add_frame(frame.data, u64::MAX, frame.key);
    }
    let mut finish = encoder.finish()?;
    while let Some(frame) = finish.next()? {
        track.add_frame(frame.data, u64::MAX, frame.key);
    }
    segment.finalize(Some(u64::MAX));
    Ok(())
}
