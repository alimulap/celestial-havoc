#![allow(unused)]

use godot::{engine::{IRigidBody2D, RigidBody2D}, prelude::*};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Bullet {
    base: Base<RigidBody2D>,
}

#[godot_api]
impl Bullet {
    #[func]
    pub fn shoot(&mut self, direction: Vector2) {
        self.base_mut().set_linear_velocity(direction * 1000f32);
    }
}

#[godot_api]
impl IRigidBody2D for Bullet {
    fn init(base: Base<RigidBody2D>) -> Self {
        Bullet { 
            base,
        }
    }

    fn ready(&mut self) {
        self.base_mut().set_gravity_scale(0f32);
    }
}
