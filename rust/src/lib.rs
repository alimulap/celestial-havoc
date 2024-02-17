use godot::prelude::*;

mod player;
mod enemy;
mod bullet;
mod camera;
mod utils;
mod test_level;

struct CelestialHavoc;

#[gdextension]
unsafe impl ExtensionLibrary for CelestialHavoc {}
