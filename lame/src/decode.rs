use crate::ffi;
use crate::Lame;
use std::os::raw::c_int;

#[derive(Debug)]
pub enum DecodeError {
    OutputBufferTooSmall,
    NoHeader,
    Unknown(c_int),
}

pub trait Decode<S> {
    fn decode(&mut self, mp3buffer: &[u8], pcm_buffer_l: &mut [S], pcm_buffer_r: &mut [S]) -> Result<usize, DecodeError>;
}

impl Decode<i16> for Lame {
    fn decode(&mut self, mp3buffer: &[u8], pcm_buffer_l: &mut [i16], pcm_buffer_r: &mut [i16]) -> Result<usize, DecodeError> {
        let retn = unsafe {
            ffi::hip_decode(self.hip,
                            &mp3buffer[0], mp3buffer.len(),
                            pcm_buffer_l.as_mut_ptr(), pcm_buffer_r.as_mut_ptr())
        };
        if retn < 0 { return Err(DecodeError::Unknown(retn)); }
        let decoded_samples = retn;

        Ok(decoded_samples as usize)
    }
}