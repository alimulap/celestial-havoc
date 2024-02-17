use godot::{engine::{ProgressBar, RigidBody2D}, prelude::*};

use crate::player::RB2D;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Enemy {
    base: Base<Node2D>,
    rb: RB2D,
    hpbar: Option<Gd<ProgressBar>>,
    health_points: u64,
    max_health_points: u64,
    counter: u64,
}

impl Enemy {
    const HPBAR_OFFSET: Vector2 = Vector2::new(-36., -64.);

    pub fn take_damage(&mut self, damage: u64) {
        self.health_points -= damage;
        self.hpbar.as_mut().unwrap().set_value(self.health_points as f64);

        if self.health_points <= 0 {
            let id = self.base().instance_id().to_variant();
            self.base_mut().emit_signal("queue_despawn".into(), &[id]);
            self.base_mut().queue_free();
        }
    }
}

#[godot_api]
impl Enemy {
    #[signal]
    fn queue_despawn();

    #[func]
    pub fn body_entered(&mut self, mut body: Gd<Node>) {
        if body.is_in_group("bullets".into()) {
            self.take_damage(10);
            body.queue_free();
        }
    }
}

#[godot_api]
impl INode2D for Enemy {
    fn init(base: Base<Node2D>) -> Self {
        Enemy {
            base,
            rb: RB2D(None),
            hpbar: None,
            health_points: 100,
            max_health_points: 100,
            counter: 0,
        }
    } 

    fn ready(&mut self) {
        let mut rb = self.base().get_node_as::<RigidBody2D>("RigidBody2D");
        rb.set_gravity_scale(0f32);
        rb.set_linear_velocity(Vector2::new(-1.0, 0.0));
        rb.set_contact_monitor(true);
        rb.set_max_contacts_reported(1);
        self.rb = RB2D(Some(rb));

        let mut hpbar = self.base().get_node_as::<ProgressBar>("HealthBar");
        hpbar.set_max(self.max_health_points as f64);
        hpbar.set_value(self.health_points as f64);
        self.hpbar = Some(hpbar);
    }

    fn process(&mut self, _delta: f64) {
        let mut velocity = self.rb.get_linear_velocity();
        velocity = velocity.lerp(Vector2::ZERO, 0.1);
        self.rb.set_linear_velocity(velocity);
        let position = self.rb.get_position();
        self.hpbar.as_mut().unwrap().set_position(position + Self::HPBAR_OFFSET);
        self.counter += 1;
        if self.counter % 60 == 0 {
            godot_print!("Enemy position: {:?}", position);
        }
    }
}
