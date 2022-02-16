use lame::Lame;
use crate::process::{Error, ProcessStereo};
use Vec;
use lame::decode::Decode;
use lame::encode::{Encode, EncodeError};

const BYTE_BUF_SIZE: usize = 12500 + 7200;
const PCM_BUF_SIZE: usize = 44100 * 2;

pub struct Mp3Processor {
    lame: Lame,
    byte_buffer: Vec<u8>,
    pcm_buffer_l: Vec<i16>,
    pcm_buffer_r: Vec<i16>,
}

impl Default for Mp3Processor {
    fn default() -> Self {
        Mp3Processor::new()
            .unwrap()
    }
}

impl Mp3Processor {
    pub fn new() -> Option<Mp3Processor> {
        match Lame::new() {
            None => None,
            Some(lame) => Some(Mp3Processor {
                lame,
                byte_buffer: vec![0; BYTE_BUF_SIZE],
                pcm_buffer_l: vec![0; PCM_BUF_SIZE],
                pcm_buffer_r: vec![0; PCM_BUF_SIZE],
            }),
        }
    }

    pub fn set_parameters(&mut self, sample_rate: u32, kilobitrate: u32) -> Result<(), Error> {
        self.lame.init_params().map_err(|_| { Error::InternalError })?;
        self.lame.set_channels(2).map_err(|_| { Error::InternalError })?;
        self.lame.set_sample_rate(sample_rate).map_err(|_| { Error::InternalError })?;
        self.lame.set_kilobitrate(kilobitrate as i32).map_err(|_| { Error::InternalError })?;
        Ok(())
    }

    pub fn process_iter(&mut self, input_buffers: (&[f32], &[f32])) -> Result<impl Iterator<Item=(f32, f32)> + '_, Error> {
        // encode into `bytes`
        let byte_size = self.lame.encode_flushing_nogap(input_buffers.0, input_buffers.1, &mut self.byte_buffer)
            .map_err(|e| match e {
                EncodeError::NoMem => Error::NoMem,
                _ => Error::InternalError,
            })?;
        let bytes = &self.byte_buffer[..byte_size];

        // decode into `samples`
        let len = self.lame.decode(bytes, &mut self.pcm_buffer_l[..], &mut self.pcm_buffer_r[..])
            .unwrap();
        let samples = self.pcm_buffer_l.iter().take(len).zip(self.pcm_buffer_r.iter().take(len))
            .map(|(&l, &r)| (
                (l as f32) / (i16::MAX as f32),
                (r as f32) / (i16::MAX as f32)
            ));
        Ok(samples)
    }
}

impl ProcessStereo for Mp3Processor {
    fn process(&mut self, input_buffers: (&[f32], &[f32]), output_buffers: (&mut [f32], &mut [f32])) -> Result<usize, Error> {
        // write into output_buffers
        let (out_l, out_r) = output_buffers;
        let output_length = self.process_iter(input_buffers)?
            .zip(out_l.iter_mut())
            .zip(out_r.iter_mut())
            .map(|(((in_l, in_r), out_l), out_r)| {
                *out_l = in_l;
                *out_r = in_r;
            })
            .count();
        Ok(output_length)
    }
}