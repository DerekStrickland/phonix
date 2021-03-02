#[macro_use]
extern crate vst;
extern crate rand;

use vst::plugin::{Info, Plugin, Category};
use vst::buffer::AudioBuffer;
use rand::random;

#[derive(Default)]
struct DelayPlugin;

impl Plugin for DelayPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "Delay Plugin".to_string(),
            unique_id: 6789,
            inputs: 0,
            outputs: 2,
            category: Category::Synth,

            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (_, mut output_buffer) = buffer.split();

        for output_channel in output_buffer.into_iter() {
            for output_sample in output_channel {
                *output_sample = (random::<f32>() - 0.5f32) * 2f32;
            }
        }
    }
}

plugin_main!(DelayPlugin);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
