use crate::data_store::textured_model_name::TexturedModelName;
use alloc::string::String;

pub struct MeshInstance {
    pub model_name: TexturedModelName
}

#[derive(Debug)]
pub struct Text {
    pub x: i32,
    pub y: i32,
    pub text: String,
    pub size: u32,
    pub color: u32,
}