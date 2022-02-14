use crate::process::Error;
use crate::ProcessStereo;

pub struct BypassProcessor {}

impl BypassProcessor {
    pub fn new() -> Self {
        BypassProcessor {}
    }
}

impl Default for BypassProcessor {
    fn default() -> Self {
        BypassProcessor::new()
    }
}

impl ProcessStereo for BypassProcessor {
    fn process(&mut self, input_buffers: (&[f32], &[f32]), output_buffers: (&mut [f32], &mut [f32])) -> Result<usize, Error> {
        if output_buffers.0.len() < input_buffers.0.len() {
            panic!("Too small output buffer")
        }
        if output_buffers.1.len() < input_buffers.1.len() {
            panic!("Too small output buffer")
        }
        if input_buffers.0.len() != input_buffers.1.len() {
            panic!("left and right input buffer must have same length!");
        }

        for (input, output) in input_buffers.0.iter().zip(output_buffers.0.iter_mut()) {
            *output = *input
        }
        for (input, output) in input_buffers.1.iter().zip(output_buffers.1.iter_mut()) {
            *output = *input
        }

        Ok(input_buffers.0.len())
    }
}