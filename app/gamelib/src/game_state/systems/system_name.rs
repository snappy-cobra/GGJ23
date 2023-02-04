use crate::game_state::GameState;
use super::audio::system_play_audio;
use super::motion::*;
use super::actions::*;
use super::render::*;

/**
 * Enumerates all systems that exist in the project.
 * Each of them can be turned into its actual function by calling `to_function()` on it.
 */
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum SystemName {
    ExitAction,
    StopAction,
    ShakeAction,
    IntegrateMotion,
    BounceBounds,
    PlayAudio,
    RenderMeshes,
    RenderText,
}

impl SystemName {
    /// Returns the function of this particular SystemName
    pub const fn to_function(&self) -> &'static dyn Fn(&mut GameState) {
        match self {
            SystemName::ExitAction => &system_exit_action,
            SystemName::StopAction => &system_stop_action,
            SystemName::ShakeAction => &system_shake_action,
            SystemName::IntegrateMotion => &system_integrate_motion,
            SystemName::BounceBounds => &system_bounce_bounds,
            SystemName::PlayAudio => &system_play_audio,
            SystemName::RenderMeshes => &system_render_meshes,
            SystemName::RenderText => &system_render_text,
        }
    }
}