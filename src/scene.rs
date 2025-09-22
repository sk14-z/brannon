use crate::{make_scene_key, panel::frame::Frame};
use std::any::Any;

pub trait SceneKey: Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn eq(&self, other: &dyn SceneKey) -> bool;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DefaultScene;

make_scene_key!(DefaultScene);

pub trait SceneKeyT: SceneKey + Copy + Clone {}
impl<T: SceneKey + Copy + Clone> SceneKeyT for T {}

pub struct Scene {
    pub(crate) key: Box<dyn SceneKey>,
    pub frame: Frame,
}

impl Scene {
    pub fn new(key: impl SceneKeyT, frame: Frame) -> Self {
        Scene {
            key: Box::new(key),
            frame,
        }
    }
}

pub(crate) struct SceneHandler {
    pub(crate) scenes: Vec<Scene>,
    pub(crate) current_pos: usize,
}

impl SceneHandler {
    pub(crate) fn new() -> Self {
        SceneHandler {
            scenes: Vec::new(),
            current_pos: 0,
        }
    }

    pub(crate) fn create_scene<T: SceneKeyT>(&mut self, key: T, frame: Frame) {
        self.scenes.push(Scene::new(key, frame));
    }

    pub(crate) fn remove_scene<T: SceneKeyT>(&mut self, key: T) -> Option<Scene> {
        if let Some(i) = self.scenes.iter().position(|s| std::ptr::eq(&*s.key, &key)) {
            Some(self.scenes.remove(i))
        } else {
            None
        }
    }

    pub(crate) fn change_scene<T: SceneKeyT>(&mut self, key: &mut T) {
        if let Some(i) = self.scenes.iter().position(|s| s.key.eq(key)) {
            self.current_pos = i;
        }
    }

    pub(crate) fn current(&mut self) -> &mut Scene {
        self.scenes.get_mut(self.current_pos).unwrap()
    }
}
