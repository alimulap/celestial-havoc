use std::collections::HashMap;

use godot::{
    engine::{utilities::randi, Label, RigidBody2D, Timer},
    prelude::*,
};

use crate::{camera::CustomCamera, enemy::Enemy, player::Player, utils::GdObj};

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct TestLevel {
    base: Base<Node2D>,
    enemy_scene: Gd<PackedScene>,
    player: GdObj<Player>,
    player_rb: GdObj<RigidBody2D>,
    camera: GdObj<CustomCamera>,
    enemy_spawn_timer: GdObj<Timer>,
    #[allow(dead_code)]
    enemies: HashMap<InstanceId, Gd<Enemy>>,
    debug_node: Option<Gd<Label>>,
    counter: u64,
}

#[godot_api]
impl TestLevel {
    #[func]
    fn spawn_enemy_near(&mut self) {
        let player_pos = self.player_rb.get_position();
        let mut enemy = self.enemy_scene.instantiate_as::<Enemy>();
        let angle = randi().rem_euclid(360) as f32;
        enemy.set_position(Vector2 {
            x: player_pos.x + 200. * angle.to_radians().cos(),
            y: player_pos.y + 200. * angle.to_radians().sin(),
        });
        let id = enemy.instance_id();
        enemy.connect(
            "queue_despawn".into(),
            self.base().callable("despawn_enemy"),
        );
        self.base_mut().add_child(enemy.clone().upcast());
        self.enemies.insert(id, enemy);
    }

    #[func]
    fn despawn_enemy(&mut self, id: InstanceId) {
        if self.enemies.remove(&id).is_none() {
            godot_print!("Failed to remove enemy with id: {}", id);
        }
    }
}

#[godot_api]
impl INode2D for TestLevel {
    fn init(base: Base<Node2D>) -> Self {
        TestLevel {
            base,
            enemy_scene: PackedScene::new_gd(),
            player: GdObj::new(),
            player_rb: GdObj::new(),
            camera: GdObj::new(),
            enemy_spawn_timer: GdObj::new(),
            enemies: HashMap::new(),
            debug_node: None,
            counter: 0,
        }
    }

    fn ready(&mut self) {
        self.enemy_scene = load("res://scenes/enemy.tscn");
        self.player = self.base().get_node_as::<Player>("Player").into();
        self.player_rb = self
            .base()
            .get_node_as::<RigidBody2D>("Player/RigidBody2D")
            .into();
        self.camera = self.base().get_node_as::<CustomCamera>("Camera").into();
        let mut enemy_spawn_timer = self.base().get_node_as::<Timer>("EnemySpawnTimer");
        enemy_spawn_timer.connect("timeout".into(), self.base().callable("spawn_enemy_near"));
        self.enemy_spawn_timer = GdObj(Some(enemy_spawn_timer));
        self.debug_node = self.base().get_node_as::<Label>("Camera/HUD/Debug").into();
        self.debug_node
            .as_mut()
            .map(|d| d.set_text("TestLevel is ready!".into()));
        if let Some(mut enemy) = self.base().try_get_node_as::<Enemy>("Enemy") {
            enemy.connect(
                "queue_despawn".into(),
                self.base().callable("despawn_enemy"),
            );
            self.enemies.insert(enemy.instance_id(), enemy);
        }

        let player_pos = self.player_rb.get_position();
        for _ in 0..10 {
            let angle = randi().rem_euclid(360) as f32;
            let distance = randi().rem_euclid(3000) as f32 + 200.;
            let position = Vector2 {
                x: player_pos.x + distance * angle.to_radians().cos(),
                y: player_pos.y + distance * angle.to_radians().sin(),
            };
            self.spawn_enemy(position);
        }
    }

    fn process(&mut self, _delta: f64) {
        self.counter += 1;
        if self.counter % 60 == 0 {
            self.debug_node
                .as_mut()
                .map(|d| d.set_text(format!("Enemy count: {}", self.enemies.len()).into()));
        }
    }
}

impl TestLevel {
    fn spawn_enemy(&mut self, position: Vector2) {
        let mut enemy = self.enemy_scene.instantiate_as::<Enemy>();
        enemy.set_position(position);
        let id = enemy.instance_id();
        enemy.connect(
            "queue_despawn".into(),
            self.base().callable("despawn_enemy"),
        );
        self.base_mut().add_child(enemy.clone().upcast());
        self.enemies.insert(id, enemy);
    }
}
