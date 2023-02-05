#[derive(Debug)]
pub struct Platform {
}

pub struct Camera {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub up_x: f32,
    pub up_y: f32,
    pub up_z: f32,
    pub lookat_x: f32,
    pub lookat_y: f32,
    pub lookat_z: f32,
}

pub enum OnAnimationFinish {
    Despawn,
    Repeat,
    RepeatBubble,
    Hand3,
    Hand2,
    Hand1,
    Hand0,
    Start
}

pub enum AnimationType {
    None,
    Test,
    Bubble,
    HandIn,
    Hand,
    HandOut,
}

pub struct Animation {
    pub duration: f32,
    pub past_time: f32,
    pub animation_type: AnimationType,
    pub on_animation_finish: OnAnimationFinish,
    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,
}