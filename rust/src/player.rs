#![allow(unused)]

use std::ops::{Deref, DerefMut};

use godot::{
    engine::{global::Key, IRigidBody2D, Label, ProgressBar, RigidBody2D},
    prelude::*,
};

use crate::bullet::Bullet;

pub struct RB2D(pub Option<Gd<RigidBody2D>>);

impl Deref for RB2D {
    type Target = RigidBody2D;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap().deref()
    }
}

impl DerefMut for RB2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut().unwrap().deref_mut()
    }
}

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Player {
    base: Base<Node2D>,
    rb: RB2D,
    hpbar: Option<Gd<ProgressBar>>,
    health_points: u64,
    max_health_points: u64,
    speed: u64,
    max_speed: u64,
    torque: f32,
    max_torque: f32,
    force: Vector2,
    counter: u64,
    bullet_scene: Gd<PackedScene>,
    debug_node: Option<Gd<Label>>,
}

#[godot_api]
impl Player {
    #[signal]
    pub fn position_changed(position: Vector2);
}

#[godot_api]
impl INode2D for Player {
    fn init(base: Base<Node2D>) -> Self {
        Player {
            base,
            rb: RB2D(None),
            hpbar: None,
            health_points: 100,
            max_health_points: 100,
            speed: 27,
            max_speed: 360,
            torque: 0f32,
            max_torque: 9000f32,
            force: Vector2::default(),
            counter: 0,
            bullet_scene: PackedScene::new_gd(),
            debug_node: None,
        }
    }

    fn ready(&mut self) {
        godot_print!("Hello, world! from Player");
        let mut rb = self.base().get_node_as::<RigidBody2D>("RigidBody2D");
        rb.set_gravity_scale(0f32);
        self.rb = RB2D(Some(rb));
        self.bullet_scene = load("res://scenes/bullet.tscn");
        let mut hpbar = self.base().get_node_as::<ProgressBar>("HealthBar");
        hpbar.set_max(self.max_health_points as f64);
        hpbar.set_value(self.health_points as f64);
        self.hpbar = Some(hpbar);
        match self
            .rb
            .try_get_node_as::<Label>("/root/root/Camera/HUD/Debug")
        {
            Some(debug_node) => {
                self.debug_node = Some(debug_node);
                godot_print!("Found DebugLabel");
            }
            None => {
                godot_print!("No DebugLabel found for Player");
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        let position = self.rb.get_position();
        self.hpbar
            .as_mut()
            .unwrap()
            .set_position(position + Self::HPBAR_OFFSET);
    }

    fn physics_process(&mut self, _delta: f64) {
        let input = Input::singleton();
        let shoot_pressed = input.is_key_pressed(Key::SPACE);
        let mut rb = self.rb.0.as_mut().unwrap();
        let mut torque = &mut self.torque;
        Self::turn(rb, &mut torque, &input);
        Self::limit_and_apply_torque(rb, torque, self.max_torque);
        let mut force = &mut self.force;
        let mut velocity = rb.get_linear_velocity();
        let angle = rb.get_rotation() - std::f32::consts::PI / 2.0;
        Self::gas(
            rb,
            &input,
            &mut force,
            &mut velocity,
            self.speed,
            self.max_speed,
            angle,
        );
        if shoot_pressed {
            Self::shoot(rb, &velocity, angle, &self.bullet_scene);
        }

        self.counter += 1;
        if self.counter % 60 == 0 {
            godot_print!("Angle: {}", angle);
            let mass = rb.get_mass();
            match self.debug_node.as_mut() {
                Some(debug_node) => {
                    debug_node.set_text(
                        format!(
                            "Force: {:?}\nTorque: {}\nAngle: {}\nVelocity: {:?}\nMass: {}",
                            force, torque, angle, velocity, mass
                        )
                        .into(),
                    );
                }
                None => {
                    godot_print!("No DebugLabel found for Player");
                }
            }
        }
    }
}

impl Player {
    const TURN_SPEED: f32 = 100.;
    const MUZZLE_POSITION: Vector2 = Vector2::new(0., -48.);
    const HPBAR_OFFSET: Vector2 = Vector2::new(-36., -64.);

    fn turn(rb: &mut Gd<RigidBody2D>, torque: &mut f32, input: &Gd<Input>) {
        let turn_left_action = input.is_key_pressed(Key::LEFT) || input.is_key_pressed(Key::A);
        let turn_right_action = input.is_key_pressed(Key::RIGHT) || input.is_key_pressed(Key::D); match (
            turn_left_action,
            turn_right_action,
            torque.is_sign_positive(),
        ) {
            (false, true, positive) => {
                if positive {
                    *torque += Self::TURN_SPEED;
                } else {
                    *torque += Self::TURN_SPEED * 2.;
                }
            }
            (true, false, positive) => {
                if positive {
                    *torque -= Self::TURN_SPEED * 2.;
                } else {
                    *torque -= Self::TURN_SPEED;
                }
            }
            (false, false, _) => {
                *torque = 0.;
                let mut angular_velocity = rb.get_angular_velocity();
                angular_velocity = angular_velocity.lerp(0., 0.1);
                rb.set_angular_velocity(angular_velocity);
            }
            _ => (),
        }
    }

    fn limit_and_apply_torque(rb: &mut Gd<RigidBody2D>, torque: &mut f32, max_torque: f32) {
        if torque.abs() < 0.1 {
            *torque = 0.;
        }
        *torque = torque.clamp(-max_torque, max_torque);
        let torque = torque;
        rb.apply_torque(*torque);
    }

    fn gas(
        rb: &mut Gd<RigidBody2D>,
        input: &Gd<Input>,
        force: &mut Vector2,
        velocity: &mut Vector2,
        speed: u64,
        max_speed: u64,
        angle: f32,
    ) {
        let gas_pressed = input.is_key_pressed(Key::W) || input.is_key_pressed(Key::UP);
        let boost_pressed = input.is_key_pressed(Key::SHIFT);
        if gas_pressed {
            force.x += angle.cos() * speed as f32;
            force.y += angle.sin() * speed as f32;
        } else {
            *force = force.lerp(Vector2::ZERO, 0.1);
        }
        if force.length() < 0.1 {
            *force = Vector2::ZERO;
        } else {
            *force =
                force.limit_length(Some(max_speed as f32 * if boost_pressed { 5. } else { 1. }));
        }
        rb.apply_central_force(*force);
        if !gas_pressed {
            *velocity = velocity.lerp(Vector2::ZERO, 0.03);
            rb.set_linear_velocity(*velocity);
        }
    }

    fn shoot(
        rb: &mut Gd<RigidBody2D>,
        velocity: &Vector2,
        angle: f32,
        bullet_scene: &Gd<PackedScene>,
    ) {
        let mut bullet = bullet_scene.instantiate_as::<Bullet>();
        bullet.set_position(Self::MUZZLE_POSITION);
        bullet.set_linear_velocity(Vector2 {
            x: velocity.x + angle.cos() * 1000.,
            y: velocity.y + angle.sin() * 1000.,
        });
        rb.add_child(bullet.upcast());
    }
}
