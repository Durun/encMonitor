#[derive(Debug)]
pub enum Error {
    // TODO: add error types
    InternalError,
    NoMem,
    Other(i32),
}

pub trait ProcessStereo {
    fn process(&mut self, input_buffers: (&[f32], &[f32]), output_buffers: (&mut [f32], &mut [f32])) -> Result<(), Error>;
}
