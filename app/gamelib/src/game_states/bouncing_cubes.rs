use hecs::World;
use num::ToPrimitive;
use ogc_rs::print;

use crate::data_store::asset_name::AssetName;
use crate::game_state::GameState;
use crate::game_state::components::audio::Audio;
use crate::game_state::components::motion::{Position, Velocity, Rotation};
use crate::game_state::components::render::MeshInstance;
use crate::game_state::components::game::*;
use crate::game_state::systems::system_name::SystemName;
use crate::servers::audio::PlayMode;
use crate::data_store::textured_model_name::TexturedModelName;

use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use crate::game_state::components::physics::SphereCollider;

/**
 * Build the bouncing cubes game state.
 */
pub fn build() -> GameState {
    let mut state = GameState::new();
    state.add_system(SystemName::GameMaster);
    state.add_system(SystemName::PlayAudio);
    state.add_system(SystemName::ExitAction);
    state.add_system(SystemName::RegisterCollider);
    state.add_system(SystemName::StopAction);
    // state.add_system(SystemName::ShakeAction);
    // state.add_system(SystemName::IntegrateMotion);
    // state.add_system(SystemName::BounceBounds);
    state.add_system(SystemName::ScoreFryingPan);
    state.add_system(SystemName::PatatoControl);
    state.add_system(SystemName::PhysicsToPosition);
    state.add_system(SystemName::RenderMeshes);
    state.add_system(SystemName::CameraUpdate);
    state.add_system(SystemName::TeleportPotatoes);
    state.add_system(SystemName::ResetLevel);

    batch_spawn_entities(&mut state.world);
    spawn_main_music(&mut state.world);
    
    //state.add_system(SystemName::DebugPhysics);
    return state;
}

/**
 * The main music is an entity with an audio component.
 */
fn spawn_main_music(world: &mut World) {
    let audio = Audio::new(AssetName::DemoMusic, PlayMode::Infinite);
    world.spawn((audio,));
}

/**
 * Spawn multiple entities in the world
 */
fn batch_spawn_entities(world: &mut World) {
    let cam_position = Position {
        x: 0.0,
        y: 27.5,
        z: 25.0,
    };
    let camera = Camera {
        r: 0x00,        g: 0x00,        b: 0x00,
        up_x: 0.0,      up_y: 1.0,      up_z: 0.0,
        lookat_x: 0.0,  lookat_y: -6.0,  lookat_z: 0.0,
    };
    world.spawn((camera, cam_position));

    let mut small_rng = SmallRng::seed_from_u64(10u64);    

    // Plate
    let plate_mesh = MeshInstance { model_name: TexturedModelName::Plate };
    let plate_position = Position{
        x: 0.0,
        y: -1.5,
        z: 0.0,
    };
    let plate_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
    world.spawn((plate_mesh, plate_position, plate_rotation));

    // Oil
    let oil_mesh = MeshInstance { model_name: TexturedModelName::OilSea };
    let oil_position = Position{
        x: 0.0,
        y: -15.0,
        z: 0.0,
    };
    let oil_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
    world.spawn((oil_mesh, oil_position, oil_rotation));


    // OilBubble
    for index in 0..20 {
        let bubble_mesh = MeshInstance { model_name: TexturedModelName::OilBubble };
        let x = (small_rng.next_u32() as f32 / u32::MAX as f32 - 0.5) * 40.0;
        let z = (small_rng.next_u32() as f32 / u32::MAX as f32 - 0.5) * 40.0;

        let bubble_position = Position{
            x: x,
            y: -6.0,
            z: z,
        };
        let bubble_animation = Animation {
            duration: 0.5 + small_rng.next_u32() as f32 / u32::MAX as f32 * 2.0,
            past_time: small_rng.next_u32() as f32 / u32::MAX as f32 * 2.0,
            animation_type: AnimationType::Bubble,
            on_animation_finish: OnAnimationFinish::RepeatBubble,
            target_x: x,
            target_y: -6.0,
            target_z: z,
        };
        let bubble_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
        world.spawn((bubble_mesh, bubble_position, bubble_animation, bubble_rotation));
    }
}
