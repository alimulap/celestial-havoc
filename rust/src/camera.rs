use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Camera2D)]
pub struct CustomCamera {
    base: Base<Camera2D>,
    target: Option<Gd<Node2D>>,
}

#[godot_api]
impl CustomCamera {
    #[func]
    pub fn set_center(&mut self, position: Vector2) {
        self.base_mut().set_position(position);
    }
}

#[godot_api]
impl ICamera2D for CustomCamera {
    fn init(base: Base<Camera2D>) -> Self {
        CustomCamera { 
            base,
            target: None,
        }
    }

    fn ready(&mut self) {
        match self.base().try_get_node_as("/root/root/Player/RigidBody2D") {
            Some(player) => {
                self.target = Some(player);
                godot_print!("Found Player");
            }
            None => {
                godot_print!("No player found for CustomCamera");
            }
        }
        //match self.base().get_parent() {
        //    Some(parent) => {
        //        match parent.get_node("Player".into()) {
        //            Some(mut player) => {
        //                player.connect(
        //                    "position_changed".into(),
        //                    Callable::from_object_method(&self.to_gd(), "set_center".to_string()),
        //                );
        //                godot_print!("Connected to Player");
        //            }
        //            None => {
        //                godot_print!("No player found for CustomCamera");
        //            }
        //        }
        //    }
        //    None => {
        //        godot_print!("No parent found for CustomCamera");
        //    }
        //}
        //godot_print!("Hello, world! from CustomCamera");
    }

    fn process(&mut self, _delta: f64) {
        let new_position = if let Some(target) = &self.target {
            let position = self.base().get_position();
            let target_position = target.get_global_position();
            position.lerp(target.get_global_position(), position.distance_to(target_position) / 500.0)
        } else {
            self.base().get_position()
        };
        self.base_mut().set_position(new_position);
    }
}

