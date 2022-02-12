mod process;
mod process_mp3;

#[macro_use]
extern crate vst;

use std::sync::Arc;
use vst::api::Supported;
use vst::buffer::AudioBuffer;
use vst::host::Host;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;
use crate::process_mp3::Mp3Processor;


struct EncMonitorParameters {
    threshold: AtomicFloat,
}

impl Default for EncMonitorParameters {
    fn default() -> Self {
        EncMonitorParameters {
            threshold: AtomicFloat::new(1.0),
        }
    }
}

impl PluginParameters for EncMonitorParameters {
    // This is what will display underneath our control.  We can
    // format it into a string that makes the most since.
    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{:.2}", self.threshold.get()),
            _ => "".to_string(),
        }
    }

    // This shows the control's name.
    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Threshold",
            _ => "",
        }
            .to_string()
    }

    // the `get_parameter` function reads the value of a parameter.
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.threshold.get(),
            _ => 0.0,
        }
    }

    // the `set_parameter` function sets the value of a parameter.
    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.threshold.set(val),
            _ => (),
        }
    }
}

#[derive(Default)]
struct EncMonitor {
    params: Arc<EncMonitorParameters>,
    processor_mp3: Mp3Processor,
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

        EncMonitor {
            params: Arc::new(EncMonitorParameters::default()),
            processor_mp3,
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

        //self.processor_mp3.process(inputs, outputs).unwrap();
    }

    // Return the parameter object. This method can be omitted if the
    // plugin has no parameters.
    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}


plugin_main!(EncMonitor);