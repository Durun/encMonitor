
use std::borrow::{Borrow, BorrowMut};
use std::fs::File;
use std::io::Write;
use hound;
use hound::{SampleFormat, WavSpec};
use lame::{Encode, Lame};

fn to_string(spec: WavSpec) -> String {
    return format!("{}ch {}Hz {}bit {}",
                   spec.channels,
                   spec.sample_rate,
                   spec.bits_per_sample,
                   match spec.sample_format {
                       SampleFormat::Int => "int",
                       SampleFormat::Float => "float",
                   }
    );
}

fn main1() {
    // read WAV
    let mut reader = hound::WavReader::open("testresources/se_saa06.wav").unwrap();
    let mut spec = reader.spec();
    println!("{}", to_string(spec));
    println!("length: {} samples", reader.len());
    println!("duration: {}", reader.duration());

    let buffer_l = reader.samples()
        .step_by(2)
        .map(|s| s.unwrap()).collect();
    reader.seek(0).unwrap();
    let buffer_r = reader.samples()
        .skip(1).step_by(2)
        .map(|s| s.unwrap()).collect();
    let buffer: (Vec<i16>, Vec<i16>) = (buffer_l, buffer_r);

    println!("Buffer: ");
    println!("length: {}, {}", buffer.0.len(), buffer.1.len());

    // encode to MP3
    let mut lame = Lame::new().unwrap();
    let kbps = 320;
    lame.init_params().unwrap();
    lame.set_channels(2).unwrap();
    lame.set_sample_rate(spec.sample_rate).unwrap();
    lame.set_kilobitrate(kbps).unwrap();

    let buf_length = kbps * 1024 / 8 * (reader.duration() as i32) / (spec.sample_rate as i32) + 1024;
    println!("buf_length = {}", buf_length);
    let mut mp3_buffer: Vec<u8> = vec![0_u8; buf_length as usize];

    let mp3_length = lame.encode(buffer.0.borrow(), buffer.1.borrow(), mp3_buffer.borrow_mut()).unwrap();


    let mut file = File::create("testresources/out.mp3").unwrap();
    let _ = mp3_buffer.split_off(mp3_length);
    file.write_all(mp3_buffer.borrow()).unwrap();
    file.flush().unwrap();

    println!("Out(mp3): ");
    println!("size: {} bytes", mp3_buffer.len());

    // decode MP3
    let (header, samples) = puremp3::read_mp3(&mp3_buffer[..]).expect("Invalid MP3");
    println!("{}Hz {}bps", header.sample_rate.hz(), header.bitrate.bps());

    spec.bits_per_sample = 32;
    spec.sample_format = SampleFormat::Float;
    let mut writer = hound::WavWriter::create("testresources/out.wav", spec).unwrap();
    for (l, r) in samples {
        writer.write_sample(l).unwrap();
        writer.write_sample(r).unwrap();
    }
    writer.flush().unwrap();


    // read WAV
    let out_reader = hound::WavReader::open("testresources/out.wav").unwrap();
    let out_spec = out_reader.spec();
    println!("Out: ");
    println!("{}", to_string(out_spec));
    println!("length: {} samples", out_reader.len());
    println!("duration: {}", out_reader.duration());
}

fn main2() {
    // read 32bit float WAV
    let mut reader = hound::WavReader::open("testresources/f32.wav").unwrap();
    let mut spec = reader.spec();
    println!("{}", to_string(spec));
    println!("length: {} samples", reader.len());
    println!("duration: {}", reader.duration());

    let buffer_l = reader.samples()
        .step_by(2)
        .map(|s| s.unwrap()).collect();
    reader.seek(0).unwrap();
    let buffer_r = reader.samples()
        .skip(1).step_by(2)
        .map(|s| s.unwrap()).collect();
    let buffer: (Vec<f32>, Vec<f32>) = (buffer_l, buffer_r);

    // encode to MP3
    let mut lame = Lame::new().unwrap();

    let kbps = 320;
    lame.init_params().unwrap();
    lame.set_channels(2).unwrap();
    lame.set_sample_rate(spec.sample_rate).unwrap();
    lame.set_kilobitrate(kbps).unwrap();

    let buf_length = kbps * 1024 / 8 * (reader.duration() as i32) / (spec.sample_rate as i32) + 1024;
    println!("buf_length = {}", buf_length);

    let mut mp3_buffer: Vec<u8> = vec![0_u8; buf_length as usize];
    let mp3_length = lame.encode(buffer.0.borrow(), buffer.1.borrow(), mp3_buffer.borrow_mut()).unwrap();
    let _ = mp3_buffer.split_off(mp3_length);
    println!("mp3_length = {}", mp3_length);

    let mut tail_buffer: Vec<u8> = vec![0_u8; buf_length as usize];
    let tail_length = lame.encode_flush_nogap(tail_buffer.borrow_mut()).unwrap();
    let _ = tail_buffer.split_off(tail_length);
    println!("tail_length = {}", tail_length);

    mp3_buffer.append(tail_buffer.borrow_mut());
    let mut file = File::create("testresources/out.mp3").unwrap();
    file.write_all(mp3_buffer.borrow()).unwrap();
    file.flush().unwrap();

    println!("Out(mp3): ");
    println!("size: {} bytes", mp3_buffer.len());

    // decode MP3
    let (header, samples) = puremp3::read_mp3(&mp3_buffer[..]).expect("Invalid MP3");
    println!("{}Hz {}bps", header.sample_rate.hz(), header.bitrate.bps());

    spec.bits_per_sample = 32;
    spec.sample_format = SampleFormat::Float;
    let mut writer = hound::WavWriter::create("testresources/out.wav", spec).unwrap();
    for (l, r) in samples {
        writer.write_sample(l).unwrap();
        writer.write_sample(r).unwrap();
    }
    writer.flush().unwrap();

    // read WAV
    let out_reader = hound::WavReader::open("testresources/out.wav").unwrap();
    let out_spec = out_reader.spec();
    println!("Out: ");
    println!("{}", to_string(out_spec));
    println!("length: {} samples", out_reader.len());
    println!("duration: {}", out_reader.duration());
}

fn main() {
    main2()
}