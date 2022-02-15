use std::os::raw::{c_int, c_void, c_ulong, c_short};

pub type LamePtr = *mut c_void;
pub type HipPtr = *mut c_void;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct mp3data_struct {
    pub header_parsed: c_int,
    pub stereo: c_int,
    pub samplerate: c_int,
    pub bitrate: c_int,
    pub mode: c_int,
    pub mode_ext: c_int,
    pub framesize: c_int,
    pub nsamp: c_ulong,
    pub totalframes: c_int,
    pub framenum: c_int,
}

#[link(name="mp3lame")]
extern "C" {
    pub fn lame_init() -> LamePtr;
    pub fn lame_close(ptr: LamePtr) -> c_int;
    pub fn lame_set_in_samplerate(ptr: LamePtr, samplerate: c_int) -> c_int;
    pub fn lame_get_in_samplerate(ptr: LamePtr) -> c_int;
    pub fn lame_set_num_channels(ptr: LamePtr, channels: c_int) -> c_int;
    pub fn lame_get_num_channels(ptr: LamePtr) -> c_int;
    pub fn lame_set_quality(ptr: LamePtr, quality: c_int) -> c_int;
    pub fn lame_get_quality(ptr: LamePtr) -> c_int;
    pub fn lame_set_brate(ptr: LamePtr, quality: c_int) -> c_int;
    pub fn lame_get_brate(ptr: LamePtr) -> c_int;
    pub fn lame_init_params(ptr: LamePtr) -> c_int;
    pub fn lame_encode_buffer(ptr: LamePtr,
                              pcm_l: *const i16, pcm_r: *const i16, pcm_numsamples: c_int,
                              mp3buf: *mut u8, mp3buf_size: c_int) -> c_int;
    pub fn lame_encode_buffer_ieee_float(ptr: LamePtr,
                                    pcm_l: *const f32, pcm_r: *const f32, pcm_numsamples: c_int,
                                    mp3buf: *mut u8, mp3buf_size: c_int) -> c_int;
    pub fn lame_encode_flush(ptr: LamePtr,
                             mp3buf: *mut u8, mp3buf_size: c_int) -> c_int;
    pub fn lame_encode_flush_nogap(ptr: LamePtr,
                                   mp3buf: *mut u8, mp3buf_size: c_int) -> c_int;
    pub fn lame_get_encoder_delay(ptr: LamePtr) -> c_int;
    pub fn lame_get_encoder_padding(ptr: LamePtr) -> c_int;

    pub fn hip_decode_init() -> HipPtr;
    pub fn hip_decode_exit(ptr: HipPtr) -> c_int;
    pub fn hip_decode1_headersB(ptr: HipPtr,
                                mp3buf: *const u8,
                                mp3buf_size: usize,
                                pcm_l: *mut c_short,
                                pcm_r: *mut c_short,
                                mp3data: *mut mp3data_struct,
                                enc_delay: *mut c_int,
                                enc_padding: *mut c_int) -> c_int;
    pub fn hip_decode1(ptr: HipPtr,
                       mp3buf: *const u8,
                       mp3buf_size: usize,
                       pcm_l: *mut c_short,
                       pcm_r: *mut c_short) -> c_int;
    pub fn hip_decode(ptr: HipPtr,
                      mp3buf: *const u8,
                      mp3buf_size: usize,
                      pcm_l: *mut c_short,
                      pcm_r: *mut c_short) -> c_int;
}