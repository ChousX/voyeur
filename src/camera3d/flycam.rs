use bevy::prelude::*;

use super::velocity::{VelocityBundle, VelocityEvent};

#[derive(Bundle)]
pub struct FlyCamBundle{
    pub velocity: VelocityBundle,
    pub camera: Camera3dBundle,
}

pub struct DefaultKeybordPlugin;
impl Plugin for DefaultKeybordPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input)
        ;
    }
}

//Todo: add the rotation of the camera
fn keyboard_input(traking: Query<(&MoveKeyboard, Entity)>, keyboard_input: Res<Input<KeyCode>>, mut output: EventWriter<VelocityEvent>){
    for (info, id) in traking.iter(){
        let mut velocity = Vec3::ZERO;
        let sensitivity = info.move_sensitivity;
        if let Some(count) = pressed(&info.forward, &keyboard_input) {
            velocity.z -= count as f32 * sensitivity;
        }
        if let Some(count) = pressed(&info.backward, &keyboard_input) {
            velocity.z += count as f32 * sensitivity;
        }

        if let Some(count) = pressed(&info.left, &keyboard_input) {
            velocity.x -= count as f32 * sensitivity;
        }
        if let Some(count) = pressed(&info.right, &keyboard_input) {
            velocity.x += count as f32 * sensitivity;
        }
        if velocity != Vec3::ZERO {
            output.send(VelocityEvent{movement: velocity, entity: id});
        }
    }
} 

type KeyBinding = Box<[KeyCode]>;
fn pressed(binding: &KeyBinding, input: &Res<Input<KeyCode>>) -> Option<u8> {
    let mut acum = 0u8;
    for key in binding.iter() {
        if input.pressed(*key) {
            acum += 1;
        }
    }
    if acum > 0 {
        Some(acum)
    } else {
        None
    }
}


#[derive(Component)]
pub struct MoveKeyboard {
    pub forward: KeyBinding,
    pub backward: KeyBinding,
    pub left: KeyBinding,
    pub right: KeyBinding,
    pub move_sensitivity: f32,
}

impl Default for MoveKeyboard {
    fn default() -> Self {
        Self {
            forward: Box::new([KeyCode::W, KeyCode::Up]),
            backward: Box::new([KeyCode::S, KeyCode::Down]),
            left: Box::new([KeyCode::A, KeyCode::Left]),
            right: Box::new([KeyCode::D, KeyCode::Right]),
            move_sensitivity: 0.25,
        }
    }
}