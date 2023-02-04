use crate::game_state::components::{render::MeshInstance, motion::Position, render::Text};
#[cfg(feature = "wii")]
use ogc_rs::prelude::Vec;
#[cfg(not(feature = "wii"))]
use std::vec::Vec;

/**
 * Simple trait for implementing the wii specific renderer.
 */
pub trait RenderServer {
    fn render_meshes(&mut self, meshes: Vec::<(&MeshInstance, &Position)>);
    fn render_text(&mut self, texts: Vec::<&Text>);
    fn render_frame(&mut self);
}