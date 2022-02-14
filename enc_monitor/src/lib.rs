mod process;
mod process_mp3;
mod process_bypass;
mod stereo_buffer;

#[macro_use]
extern crate vst;

use std::sync::Arc;
use vst::api::Supported;
use vst::buffer::AudioBuffer;
use vst::host::Host;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;
use crate::process::ProcessStereo;
use crate::process_bypass::BypassProcessor;
use crate::process_mp3::Mp3Processor;
use crate::stereo_buffer::StereoBuffer;

const DELAY_SAMPLES: usize = 44100;


struct EncMonitorParameters {
    bypass: AtomicFloat,
}

impl Default for EncMonitorParameters {
    fn default() -> Self {
        EncMonitorParameters {
            bypass: AtomicFloat::new(0.0),
        }
    }
}

impl PluginParameters for EncMonitorParameters {
    // This is what will display underneath our control.  We can
    // format it into a string that makes the most since.
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.bypass.get()),
            _ => "".to_string(),
        }
    }

    // This shows the control's name.
    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Bypass",
            _ => "",
        }
            .to_string()
    }

    // the `get_parameter` function reads the value of a parameter.
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.bypass.get(),
            _ => 0.0,
        }
    }

    // the `set_parameter` function sets the value of a parameter.
    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.bypass.set(val),
            _ => (),
        }
    }
}

#[derive(Default)]
struct EncMonitor {
    params: Arc<EncMonitorParameters>,
    processor_bypass: BypassProcessor,
    processor_mp3: Mp3Processor,
    delay_buffer: StereoBuffer<f32>,
}

impl Plugin for EncMonitor {
    fn get_info(&self) -> Info {
        Info {
            name: "EncMonitor".to_string(),
            category: Category::Effect,
            unique_id: 20220212,

            inputs: 2,
            outputs: 2,

            parameters: 1,

            initial_delay: DELAY_SAMPLES as i32,

            ..Default::default()
        }
    }
    fn new(_host: HostCallback) -> Self {
        println!("Initializing enc_monitor...");

        let time_info = _host.get_time_info(!0)// TODO
            .unwrap();

        let mut processor_mp3 = Mp3Processor::new()
            .unwrap();
        processor_mp3.set_parameters(44100, 320)
            .unwrap();

        println!("  Sample Rate: {}Hz", time_info.sample_rate); // 0Hz

        let mut delay_buffer = StereoBuffer::new(DELAY_SAMPLES * 2);
        delay_buffer.enqueue_padding(DELAY_SAMPLES);

        EncMonitor {
            params: Arc::new(EncMonitorParameters::default()),
            processor_bypass: BypassProcessor::default(),
            processor_mp3,
            delay_buffer,
        }
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::No,
            _ => Supported::Maybe,
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();
        let inputs = (&inputs[0], &inputs[1]);
        let mut outputs = outputs.split_at_mut(1);
        let outputs = (&mut outputs.0[0], &mut outputs.1[0]);

        if 0.5 < self.params.bypass.get() {
            // bypass
            for (l, r) in self.processor_bypass.process_iter(inputs).unwrap() {
                self.delay_buffer.enqueue((l, r));
            }
        } else {
            // process
            for (l, r) in self.processor_mp3.process_iter(inputs).unwrap() {
                self.delay_buffer.enqueue((l, r));
            }
        }

        let len = inputs.0.len();
        let (buf_iter_l, buf_iter_r, _) = self.delay_buffer.dequeue(len);
        for (input, output) in buf_iter_l.zip(outputs.0.iter_mut()) {
            *output = input
        }
        for (input, output) in buf_iter_r.zip(outputs.1.iter_mut()) {
            *output = input
        }
    }

    // Return the parameter object. This method can be omitted if the
    // plugin has no parameters.
    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}


plugin_main!(EncMonitor);