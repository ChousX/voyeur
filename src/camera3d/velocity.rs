use super::{friction::Friction};

use bevy::prelude::*;

pub struct VelocityPlugin;
impl Plugin for VelocityPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_event::<VelocityEvent>()
            .add_system(velocity_move)
            ;   
    }
}

#[derive(Bundle)]
pub struct VelocityBundle{
    pub velocity: Velocity,
    pub friction: Friction
}

impl Default for VelocityBundle{
    fn default() -> Self {
        Self { 
            velocity: Velocity::default(), 
            friction:  Friction(0.20)
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Velocity{
    data: Vec3
}

impl Velocity{
    //friction still feells funny
    pub fn update<F: Into<f32>>(&mut self, friction: F){
        let friction = friction.into();

        if self.data.x.abs() >= friction{
            if self.data.x > 0.0{
                //+
                self.data.x -= friction
            } else {
                //-
                self.data.x += friction
            }
        } else {
            self.data.x = 0.0;
        }


        if self.data.z.abs() >= friction{
            if self.data.z > 0.0{
                //+
                self.data.z -= friction
            } else {
                //-
                self.data.z += friction
            }
        } else {
            self.data.z = 0.0;
        }
    }
    
    pub fn get(&self) -> Vec3{
        self.data
    }

    pub fn add(&mut self, input: Vec3){
        self.data += input;
    }
}

impl From<Vec3> for Velocity{
    fn from(input: Vec3) -> Self {
        Self{data: input}
    }
}

pub struct VelocityEvent{
    pub movement: Vec3,
    pub entity: Entity,
}

fn velocity_move(mut events: EventReader<VelocityEvent>, mut query: Query<(&mut Velocity, &Friction, &mut Transform, Entity, )>, time: Res<Time>,){
    //updating velocity from events
    for event in events.iter(){
        let id = event.entity;
        if let Ok((mut velocity, _, _, _)) = query.get_mut(id){
            velocity.add(event.movement);
        }
    }

    //using velocity to update positions
    for (mut velocity, Friction(friction), mut transform, _) in query.iter_mut(){
        transform.translation += velocity.get() * time.delta_seconds();
        //aplying friction
        velocity.update(*friction * time.delta_seconds());
    }
}

