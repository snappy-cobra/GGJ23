use hecs::World;

use crate::game_state::GameState;
use crate::game_state::components::motion::Position;
use crate::game_state::components::render::MeshInstance;
use crate::game_state::systems::system_name::SystemName;
use crate::data_store::textured_model_name::TexturedModelName;

use rand::rngs::SmallRng;

/**
 * Build the bouncing cubes game state.
 */
pub fn build() -> GameState {
    let mut state = GameState::new();
    state.add_system(SystemName::RenderMeshes);
    state.add_system(SystemName::MovingPlatform);
    batch_spawn_entities(&mut state.world, 10);
    return state;
}

/**
 * Spawn multiple entities in the world
 */
fn batch_spawn_entities(world: &mut World, _n: i32) {
    let position = Position {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let mesh_instance = MeshInstance { model_name: TexturedModelName::Plate };


    world.spawn((position, mesh_instance));

}
