use godot::prelude::*;

mod player;
mod enemy;
mod bullet;
mod camera;
mod utils;

struct CelestialHavoc;

#[gdextension]
unsafe impl ExtensionLibrary for CelestialHavoc {}
