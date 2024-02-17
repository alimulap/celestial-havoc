use std::ops::Deref;

use godot::obj::{Gd, GodotClass};

pub struct GdObj<T: GodotClass>(pub Option<Gd<T>>);

impl<T: GodotClass> Deref for GdObj<T> {
    type Target = Gd<T>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

impl<T: GodotClass> GdObj<T> {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn as_ref(&self) ->&Gd<T> {
        self.0.as_ref().unwrap()
    }

    #[allow(dead_code)]
    pub fn as_mut(&mut self) -> &mut Gd<T> {
        self.0.as_mut().unwrap()
    }
}

impl<T: GodotClass> Default for GdObj<T> {
    fn default() -> Self {
        GdObj(None)
    }
}

impl<T: GodotClass> From<Gd<T>> for GdObj<T> {
    fn from(gd: Gd<T>) -> Self {
        GdObj(Some(gd))
    }
}

