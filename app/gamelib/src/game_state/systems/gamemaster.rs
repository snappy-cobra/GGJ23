use crate::game_state::components::render::MeshInstance;
use crate::game_state::*;
use crate::game_state::components::motion::*;
use crate::game_state::components::game::*;
use crate::data_store::textured_model_name::TexturedModelName;
use alloc::vec::Vec;
use alloc::vec;
use hecs::{DynamicBundle, Entity, Component};
use ogc_rs::print;
use micromath::F32Ext;
use rand::rngs::SmallRng;
use rand::RngCore;
use rand::SeedableRng;
use crate::game_state::components::physics::SphereCollider;

/**
 * Move the Platform.
 */
pub fn system_moving_platform(state: &mut GameState) {
    
}

pub fn system_gamemaster(state: &mut GameState) {
    system_animation(state);
    system_camera_movement(state);
    system_game_start(state);
    system_game_finish(state);
}

pub fn system_game_start(state: &mut GameState) {
    match state.playmode {
        PlayMode::Playing | PlayMode::Hands | PlayMode::Finish => {}
        PlayMode::Selection => {
            let tmp_position = Position{
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            let tmp_animation = Animation {
                duration: 0.0,
                past_time: 0.0,
                animation_type: AnimationType::None,
                on_animation_finish: OnAnimationFinish::Hand3,
                target_x: 0.0,
                target_y: 0.0,
                target_z: 0.0,
            };
    
            state.world.spawn((tmp_position, tmp_animation));
            state.playmode = PlayMode::Hands; // TODO: BUTTON click
        }
    }
}

pub fn system_game_finish(state: &mut GameState) {
    let mut winner: bool = false;
    for (_id, (fry, animation, pos)) in state.world.query_mut::<(&mut FryAssignment, &mut Animation, &mut Position)>() {
        if fry.score > 20 {
            state.playmode = PlayMode::Finish; // TODO: BUTTON click
            animation.duration = 5.0;
            animation.past_time = 0.0;
            animation.animation_type = AnimationType::FryerWin;
            animation.on_animation_finish= OnAnimationFinish::Restart;
            animation.target_x = pos.x;
            animation.target_y = pos.y+5.0;
            animation.target_z = pos.z;            
            winner = true;
        }
    }

    if winner {
        let mut to_remove: Vec<Entity> = Vec::new();

        for (id, fry) in state.world.query_mut::<(&mut FryAssignment)>() {
            if fry.score < 20 {
                to_remove.push(id);
            }
        }

        for id in to_remove.into_iter() {
            state.world.despawn(id);
        }
    }

    for (id, (pos, plat)) in state.world.query_mut::<(&mut Position, &mut Platform)>() {
        pos.y = 99999.0;
    }
}

pub fn system_camera_movement(state: &mut GameState) {
    for (_id, (_pos, _camera)) in state.world.query_mut::<(&mut Position, &mut Camera)>() {
        // pos.y -= state.changes.delta_time.as_secs_f32();
    }
}

pub fn lerp(a: f32, b: f32, t:f32) -> f32 {
    return a + (b-a) * t; 
}

pub fn system_animation(state: &mut GameState) {
    let mut to_remove: Vec<Entity> = Vec::new();
    let mut to_add: Vec<(Position, Rotation, Animation, MeshInstance)> = Vec::new();
    let mut startPlaying: bool = false;
    

    for (id, (pos, animation)) in state.world.query_mut::<(&mut Position, &mut Animation)>() {
        animation.past_time += state.changes.delta_time.as_secs_f32();
        let movement_size = 15.0;
        let movement_radius = 13.5;
        let delay = 0.2;
        match animation.animation_type {
            AnimationType::None => {}
            AnimationType::Test => {
                pos.x += state.changes.delta_time.as_secs_f32();
                pos.y += state.changes.delta_time.as_secs_f32();
                pos.z += state.changes.delta_time.as_secs_f32();
            }
            AnimationType::Bubble => {
                pos.y += state.changes.delta_time.as_secs_f32();
            }
            AnimationType::FryerSpin0 => {
                pos.x = (animation.past_time * delay).sin() * movement_radius;
                pos.z = (animation.past_time * delay).cos() * movement_radius;
            }
            AnimationType::Fryer0 => {
                pos.x = (animation.past_time * 0.35 + 0.01).sin() * movement_size + (animation.past_time * 1.07 + 0.03).sin() * 2.0 + (animation.past_time * 1.47 + 5.31).sin();
                pos.z = (animation.past_time * 0.23 + 0.21).cos() * movement_size + (animation.past_time * 1.13 + 0.43).cos() * 2.0 + (animation.past_time * 1.83 + 1.84).cos();
            }
            AnimationType::Fryer1 => {
                pos.x = (animation.past_time * 0.37 + 0.45).sin() * movement_size + (animation.past_time * 1.17 + 0.03).sin() * 2.0 + (animation.past_time * 1.47 + 5.31).sin();
                pos.z = (animation.past_time * 0.25 + 8.41).cos() * movement_size + (animation.past_time * 1.43 + 0.43).cos() * 2.0 + (animation.past_time * 1.83 + 1.84).cos();
            }
            AnimationType::Fryer2 => {
                pos.x = (animation.past_time * 0.22 + 5.11).sin() * movement_size + (animation.past_time * 1.14 + 0.03).sin() * 2.0 + (animation.past_time * 1.47 + 5.31).sin();
                pos.z = (animation.past_time * 0.42 + 2.53).cos() * movement_size + (animation.past_time * 1.15 + 0.43).cos() * 2.0 + (animation.past_time * 1.83 + 1.84).cos();
            }
            AnimationType::Fryer3 => {
                pos.x = (animation.past_time * 0.32 + 9.01).sin() * 10.0 + (animation.past_time * 1.15 + 0.03).sin() * 2.0 + (animation.past_time * 1.47 + 5.31).sin();
                pos.z = (animation.past_time * 0.43 + 2.32).cos() * 10.0 + (animation.past_time * 1.19 + 0.43).cos() * 2.0 + (animation.past_time * 1.83 + 1.84).cos();
            }
            AnimationType::HandIn | AnimationType::HandOut | AnimationType::FryerWin => {
                let mut t = animation.past_time / animation.duration;
                pos.x = lerp(pos.x, animation.target_x, t);
                pos.y = lerp(pos.y, animation.target_y, t);
                pos.z = lerp(pos.z, animation.target_z, t);
            }
            AnimationType::Hand => {
                pos.y += state.changes.delta_time.as_secs_f32();
            }
        }
        
        if animation.past_time >= animation.duration {
            match  animation.on_animation_finish {
                OnAnimationFinish::Repeat => { 
                    animation.past_time -= animation.duration; 
                    pos.x = animation.target_x; 
                    pos.y = animation.target_y; 
                    pos.z = animation.target_z; 
                }
                OnAnimationFinish::Fryer => {}
                OnAnimationFinish::RepeatBubble => { 
                    animation.past_time -= animation.duration; 
                    let tmp = pos.x;
                    pos.x = pos.z; 
                    pos.y = animation.target_y; 
                    pos.z = tmp; 
                }
                OnAnimationFinish::Despawn => { to_remove.push(id) }

                OnAnimationFinish::Hand3 => {
                    let hand_mesh = MeshInstance { model_name: TexturedModelName::HandThree };
                    let hand_position = Position{
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 1.5,
                        past_time: 0.0,
                        animation_type: AnimationType::HandIn,
                        on_animation_finish: OnAnimationFinish::Hand2,
                        target_x: 1.0,
                        target_y: 15.0,
                        target_z: 15.0,
                    };
                    let hand_rotation = Rotation { x: 0.0, y: 90.0, z: 10.0 };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh)); 
                    to_remove.push(id); 
                } 

                OnAnimationFinish::Hand2 => { 
                    print!("switch to hand 2");
                    let hand_position = Position {
                        x: animation.target_x, y: animation.target_y, z: animation.target_z,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0, y: 90.0, z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 1.0,
                        past_time: 0.0,
                        animation_type: AnimationType::None,
                        on_animation_finish: OnAnimationFinish::Hand1,
                        target_x: pos.x, target_y: pos.y, target_z: pos.z, 
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandTwo };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Hand1 => { 
                    print!("switch to hand 1");
                    let hand_position = Position {
                        x: animation.target_x, y: animation.target_y, z: animation.target_z,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0, y: 90.0, z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 1.0,
                        past_time: 0.0,
                        animation_type: AnimationType::None,
                        on_animation_finish: OnAnimationFinish::Hand0,
                        target_x: pos.x, target_y: pos.y, target_z: pos.z, 
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandOne };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Hand0 => { 
                    print!("switch to hand 0");
                    let hand_position = Position {
                        x: animation.target_x, y: animation.target_y, z: animation.target_z,
                    };
                    let hand_rotation = Rotation {
                        x: 0.0, y: 90.0, z: 0.0,
                    };
                    let hand_animation = Animation {
                        duration: 0.5,
                        past_time: 0.0,
                        animation_type: AnimationType::HandOut,
                        on_animation_finish: OnAnimationFinish::Start,
                        target_x: 0.0, target_y: pos.y -10.0, target_z: 40.0, 
                    };
                    let hand_mesh_instance = MeshInstance { model_name: TexturedModelName::HandFist };
                    to_add.push((hand_position, hand_rotation, hand_animation, hand_mesh_instance)); 
                    to_remove.push(id); 
                }

                OnAnimationFinish::Start => { 
                    print!("switch to Start");
                    startPlaying = true;
                    to_remove.push(id); 
                }

                OnAnimationFinish::Restart => {
                    let mut server_provider = state.server_provider.as_ref().unwrap().borrow_mut();
                    server_provider.render_server.reset_world();
                    state.next_state = Some(GameStateName::BouncingCubes);
                }
            }
        }
    }

    for id in to_remove.into_iter() {
        state.world.despawn(id);
    }
    for comps in to_add.into_iter() {
        state.world.spawn(comps);
    }

    if startPlaying {
        state.playmode = PlayMode::Playing;

        let offset: f32 = 30.0;

        let y = -10.0;
        //Creating fryingpans
        let fry_0_mesh = MeshInstance { model_name: TexturedModelName::FryPanBlack };
        let fry_0_position = Position{ x: 0.0, y: y, z: 0.0,};
        let fry_0_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
        let fry_0_assignment = FryAssignment{id: 0, score: 0};
        let fry_0_animation = Animation {
            duration: 5.0,
            past_time: 0.25 * offset,
            animation_type: AnimationType::FryerSpin0,
            on_animation_finish: OnAnimationFinish::Fryer,
            target_x: 0.0,
            target_y: y,
            target_z: 0.0,
        };
        state.world.spawn((fry_0_mesh, fry_0_position, fry_0_rotation, fry_0_assignment, fry_0_animation));

        let fry_1_mesh = MeshInstance { model_name: TexturedModelName::FryPanWhite };
        let fry_1_position = Position{ x: 0.0, y: y, z: 0.0,};
        let fry_1_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
        let fry_1_assignment = FryAssignment{id: 1, score: 0};
        let fry_1_animation = Animation {
            duration: 5.0,
            past_time: 0.5 * offset,
            animation_type: AnimationType::FryerSpin0,
            on_animation_finish: OnAnimationFinish::Fryer,
            target_x: 0.0,
            target_y: y,
            target_z: 0.0,
        };
        state.world.spawn((fry_1_mesh, fry_1_position, fry_1_rotation, fry_1_assignment, fry_1_animation));

        let fry_2_mesh = MeshInstance { model_name: TexturedModelName::FryPanBlue };
        let fry_2_position = Position{ x: 0.0, y: y, z: 0.0,};
        let fry_2_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
        let fry_2_assignment = FryAssignment{id: 2, score: 0};
        let fry_2_animation = Animation {
            duration: 5.0,
            past_time: 0.75 * offset,
            animation_type: AnimationType::FryerSpin0,
            on_animation_finish: OnAnimationFinish::Fryer,
            target_x: 0.0,
            target_y: y,
            target_z: 0.0,
        };
        state.world.spawn((fry_2_mesh, fry_2_position, fry_2_rotation, fry_2_assignment, fry_2_animation));

        let fry_3_mesh = MeshInstance { model_name: TexturedModelName::FryPanRed };
        let fry_3_position = Position{ x: 0.0, y: y, z: 0.0,};
        let fry_3_rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
        let fry_3_assignment = FryAssignment{id: 3, score: 0};
        let fry_3_animation = Animation {
            duration: 5.0,
            past_time: 1.0 * offset,
            animation_type: AnimationType::FryerSpin0,
            on_animation_finish: OnAnimationFinish::Fryer,
            target_x: 0.0,
            target_y: y,
            target_z: 0.0,
        };
        state.world.spawn((fry_3_mesh, fry_3_position, fry_3_rotation, fry_3_assignment, fry_3_animation));

        let mut small_rng = SmallRng::seed_from_u64(10u64);
        for index in 0..20 {
            const ROW_WIDTH: i32 = 10;
            let pos_x: f32 = (index % ROW_WIDTH) as f32;
            let pos_z: f32 = (index / ROW_WIDTH) as f32;

            let position = Position {
                x: pos_x,
                y: 0.0,
                z: pos_z,
            };
            let velocity = Velocity {
                x: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
                y: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
                z: small_rng.next_u32() as f32 / u32::MAX as f32 * 0.1,
            };
            let rotation = Rotation { x: 0.0, y: 0.0, z: 0.0 };
            
            let mesh_instance = MeshInstance { model_name: TexturedModelName::Potato };
            let sphere_collider = SphereCollider{radius: 1.0, gravity: true, body_index: 0, has_been_registered: false};
            let controller_assignment = ControllerAssignment{
                id: 0,
            };

            state.world.spawn((mesh_instance, position, velocity, rotation, sphere_collider, controller_assignment));
        }
    }
}