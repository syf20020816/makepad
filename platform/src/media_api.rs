use crate::{
    audio::{AudioDeviceId, AudioTime, AudioBuffer},
    midi::*,
};

pub trait CxMediaApi {
    fn midi_input(&mut self) -> MidiInput;
    fn midi_output(&mut self) -> MidiOutput;
    
    fn use_midi_inputs(&mut self, ports:&[MidiPortId]);
    fn use_midi_outputs(&mut self, ports:&[MidiPortId]);
    
    fn use_audio_inputs(&mut self, devices:&[AudioDeviceId]);
    fn use_audio_outputs(&mut self, devices:&[AudioDeviceId]);
    
    fn audio_output<F>(&mut self, index:usize, f: F) where F: FnMut(AudioDeviceId, AudioTime, &mut AudioBuffer) + Send  + 'static;
    fn audio_input<F>(&mut self, index:usize, f: F) where F: FnMut(AudioDeviceId, AudioTime, AudioBuffer)->AudioBuffer + Send  + 'static;
} 