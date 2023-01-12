use {
    crate::{
        makepad_live_id::{LiveId, FromLiveId},
    }
};

pub const MAX_AUDIO_DEVICE_INDEX: usize = 32;

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct AudioDeviceId(pub LiveId);

#[derive(Clone, Debug)]
pub struct AudioDeviceDesc {
    pub device_id: AudioDeviceId,
    pub device_type: AudioDeviceType,
    pub is_default: bool,
    pub channels: usize,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct AudioDevicesEvent{
    pub descs: Vec<AudioDeviceDesc>,
}

impl AudioDevicesEvent{
    pub fn default_input(&self)->Vec<AudioDeviceId>{
        for d in &self.descs{
            if d.is_default && d.device_type.is_input(){
                return vec![d.device_id]
            }
        }
        Vec::new()
    }
    pub fn default_output(&self)->Vec<AudioDeviceId>{
        for d in &self.descs{
            if d.is_default && d.device_type.is_output(){
                return vec![d.device_id]
            }
        }
        Vec::new()
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum AudioDeviceType {
    Input,
    Output,
}

impl AudioDeviceType{
    pub fn is_input(&self)->bool{
        match self{
            AudioDeviceType::Input=>true,
            _=>false
        }
    }
    pub fn is_output(&self)->bool{
        match self{
            AudioDeviceType::Output=>true,
            _=>false
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AudioTime {
    pub sample_time: f64,
    pub host_time: u64,
    pub rate_scalar: f64,
}

#[derive(Clone, Default)]
pub struct AudioBuffer {
    pub data: Vec<f32>,
    pub final_size: bool,
    pub frame_count: usize,
    pub channel_count: usize
}

impl AudioBuffer {
    pub fn from_data(data: Vec<f32>, channel_count: usize) -> Self {
        let frame_count = data.len() / channel_count;
        Self {
            data,
            final_size: false,
            frame_count,
            channel_count
        }
    }
    
    pub fn from_i16(inp: &[i16], channel_count: usize) -> Self {
        let mut data = Vec::new();
        data.resize(inp.len(), 0.0);
        let frame_count = data.len() / channel_count;
        for i in 0..data.len() {
            data[i] = (inp[i] as f32) / 32767.0;
        }
        Self {
            data,
            final_size: false,
            frame_count,
            channel_count
        }
    }
    
    pub fn make_single_channel(&mut self) {
        self.data.resize(self.frame_count, 0.0);
        self.channel_count = 1;
    }
    
    pub fn into_data(self) -> Vec<f32> {
        self.data
    }
    
    pub fn to_i16(&self) -> Vec<i16> {
        let mut out = Vec::new();
        out.resize(self.data.len(), 0);
        for i in 0..self.data.len() {
            let f = (self.data[i] * 32767.0).max(std::i16::MIN as f32).min(std::i16::MAX as f32);
            out[i] = f as i16;
        }
        out
    }
    
    pub fn new_with_size(frame_count: usize, channel_count: usize) -> Self {
        let mut ret = Self::default();
        ret.resize(frame_count, channel_count);
        ret
    }
    
    pub fn new_like(like: &AudioBuffer) -> Self {
        let mut ret = Self::default();
        ret.resize_like(like);
        ret
    }
    
    pub fn frame_count(&self) -> usize {self.frame_count}
    pub fn channel_count(&self) -> usize {self.channel_count}
    
    
    pub fn copy_from(&mut self, like: &AudioBuffer) -> &mut Self {
        self.resize(like.frame_count(), like.channel_count());
        self.data.copy_from_slice(&like.data);
        self
    }
    
    pub fn resize_like(&mut self, like: &AudioBuffer) -> &mut Self {
        self.resize(like.frame_count(), like.channel_count());
        self
    }
    
    pub fn resize(&mut self, frame_count: usize, channel_count: usize) {
        if self.frame_count != frame_count || self.channel_count != channel_count {
            if self.final_size {
                panic!("Audiobuffer is set to 'final size' and resize is different");
            }
            self.frame_count = frame_count;
            self.channel_count = channel_count;
            self.data.resize(frame_count * channel_count as usize, 0.0);
        }
    }
    
    pub fn clear_final_size(&mut self) {
        self.final_size = false;
    }
    
    pub fn set_final_size(&mut self) {
        self.final_size = true;
    }
    
    pub fn stereo_mut(&mut self) -> (&mut [f32], &mut [f32]) {
        if self.channel_count != 2 {panic!()}
        self.data.split_at_mut(self.frame_count)
    }
    
    pub fn stereo(&self) -> (&[f32], &[f32]) {
        if self.channel_count != 2 {panic!()}
        self.data.split_at(self.frame_count)
    }
    
    pub fn channel_mut(&mut self, channel: usize) -> &mut [f32] {
        &mut self.data[channel * self.frame_count..(channel + 1) * self.frame_count]
    }
    
    pub fn channel(&self, channel: usize) -> &[f32] {
        &self.data[channel * self.frame_count..(channel + 1) * self.frame_count]
    }
    
    pub fn zero(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0.0;
        }
    }
}
