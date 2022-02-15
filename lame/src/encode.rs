use crate::ffi;
use crate::Lame;
use std::os::raw::c_int;


#[derive(Debug)]
pub enum EncodeError {
    OutputBufferTooSmall,
    NoMem,
    InitParamsNotCalled,
    PsychoAcousticError,
    Unknown(c_int),
}

fn handle_encode_error(retn: c_int) -> Result<usize, EncodeError> {
    match retn.into() {
        -1 => Err(EncodeError::OutputBufferTooSmall),
        -2 => Err(EncodeError::NoMem),
        -3 => Err(EncodeError::InitParamsNotCalled),
        -4 => Err(EncodeError::PsychoAcousticError),
        _ => {
            if retn < 0 {
                Err(EncodeError::Unknown(retn))
            } else {
                Ok(retn as usize)
            }
        }
    }
}

fn int_size(sz: usize) -> c_int {
    if sz > c_int::MAX as usize {
        panic!("converting to c_int would overflow");
    }

    sz as c_int
}

pub trait Encode<S> {
    /// Encodes PCM data into MP3 frames. The `pcm_left` and `pcm_right`
    /// buffers must be of the same length, or this function will panic.
    fn encode(&mut self, pcm_left: &[S], pcm_right: &[S], mp3_buffer: &mut [u8]) -> Result<usize, EncodeError>;

    fn flush(&mut self, mp3_buffer: &mut [u8]) -> Result<usize, EncodeError>;
    fn flush_nogap(&mut self, mp3_buffer: &mut [u8]) -> Result<usize, EncodeError>;

    fn encode_flushing(&mut self, pcm_left: &[S], pcm_right: &[S], mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let encoded_len = self.encode(pcm_left, pcm_right, mp3_buffer)?;
        let tail_buffer = &mut mp3_buffer[encoded_len..];
        let tail_length = self.flush(tail_buffer)?;
        Ok(encoded_len + tail_length)
    }
    fn encode_flushing_nogap(&mut self, pcm_left: &[S], pcm_right: &[S], mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let encoded_len = self.encode(pcm_left, pcm_right, mp3_buffer)?;
        let tail_buffer = &mut mp3_buffer[encoded_len..];
        let tail_length = self.flush_nogap(tail_buffer)?;
        Ok(encoded_len + tail_length)
    }
}

impl Encode<i16> for Lame {
    fn encode(&mut self, pcm_left: &[i16], pcm_right: &[i16], mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        if pcm_left.len() != pcm_right.len() {
            panic!("left and right channels must have same number of samples!");
        }
        let retn = unsafe {
            ffi::lame_encode_buffer(self.ptr,
                                    pcm_left.as_ptr(), pcm_right.as_ptr(), int_size(pcm_left.len()),
                                    mp3_buffer.as_mut_ptr(), int_size(mp3_buffer.len()))
        };
        handle_encode_error(retn)
    }

    fn flush(&mut self, mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let retn = unsafe {
            ffi::lame_encode_flush(self.ptr, mp3_buffer.as_mut_ptr(), int_size(mp3_buffer.len()))
        };
        handle_encode_error(retn)
    }

    fn flush_nogap(&mut self, mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let retn = unsafe {
            ffi::lame_encode_flush_nogap(self.ptr, mp3_buffer.as_mut_ptr(), int_size(mp3_buffer.len()))
        };
        handle_encode_error(retn)
    }
}

impl Encode<f32> for Lame {
    fn encode(&mut self, pcm_left: &[f32], pcm_right: &[f32], mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        if pcm_left.len() != pcm_right.len() {
            panic!("left and right channels must have same number of samples!");
        }
        let retn = unsafe {
            ffi::lame_encode_buffer_ieee_float(self.ptr,
                                               pcm_left.as_ptr(), pcm_right.as_ptr(), int_size(pcm_left.len()),
                                               mp3_buffer.as_mut_ptr(), int_size(mp3_buffer.len()))
        };
        handle_encode_error(retn)
    }

    fn flush(&mut self, mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let retn = unsafe {
            ffi::lame_encode_flush(self.ptr, mp3_buffer.as_mut_ptr(), int_size(mp3_buffer.len()))
        };
        handle_encode_error(retn)
    }

    fn flush_nogap(&mut self, mp3_buffer: &mut [u8]) -> Result<usize, EncodeError> {
        let retn = unsafe {
            ffi::lame_encode_flush_nogap(self.ptr, mp3_buffer.as_mut_ptr(), int_size(mp3_buffer.len()))
        };
        handle_encode_error(retn)
    }
}