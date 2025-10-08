use crate::{make_scene_key, panel::frame::Frame};
use std::any::Any;

pub trait SceneKey: Any + 'static {
    fn as_any(&self) -> &dyn Any;

    fn eq(&self, other: &dyn SceneKey) -> bool;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

pub struct SceneHandler {
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

    pub fn add<T: SceneKeyT>(&mut self, key: T, frame: Frame) {
        self.scenes.push(Scene::new(key, frame));
    }

    pub fn remove<T: SceneKeyT>(&mut self, key: T) -> Option<Scene> {
        if let Some(i) = self.scenes.iter().position(|s| key.eq(&*s.key)) {
            Some(self.scenes.remove(i))
        } else {
            None
        }
    }

    pub fn set<T: SceneKeyT>(&mut self, key: &mut T) {
        if let Some(i) = self.scenes.iter().position(|s| s.key.eq(key)) {
            self.current_pos = i;
        }
    }

    pub fn current(&mut self) -> &mut Scene {
        self.scenes.get_mut(self.current_pos).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        make_scene_key,
        panel::frame::Frame,
        scene::{DefaultScene, Scene, SceneHandler, SceneKey},
    };

    // Helper keys for testing
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct keyA(u32);
    make_scene_key!(keyA);

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    struct keyB(u32);
    make_scene_key!(keyB);

    #[test]
    fn scene_key_eq_same_type_same_value() {
        let a1 = keyA(10);
        let a2 = keyA(10);
        assert!(SceneKey::eq(&a1, &a2));
    }

    #[test]
    fn scene_key_eq_same_type_diff_value() {
        let a1 = keyA(10);
        let a2 = keyA(11);
        assert!(!SceneKey::eq(&a1, &a2));
    }

    #[test]
    fn scene_key_eq_different_types() {
        let a = keyA(5);
        let b = keyB(5);
        assert!(!SceneKey::eq(&a, &b));
    }

    #[test]
    fn scene_handler_add_and_set_current() {
        let mut handler = SceneHandler::new();
        let f1 = Frame::new(None);
        let f2 = f1.clone();

        let mut k1 = keyA(1);
        let mut k2 = keyA(2);

        handler.add(k1, f1);
        handler.add(k2, f2);

        // Initially current should be first added
        {
            let cur = handler.current();
            // its key should equal k1
            assert!(cur.key.eq(&k1));
        }

        // Switch to second
        handler.set(&mut k2);
        {
            let cur = handler.current();
            assert!(cur.key.eq(&k2));
        }
    }

    #[test]
    fn scene_handler_with_default_scene() {
        let mut handler = SceneHandler::new();
        handler.add(DefaultScene, Frame::new(None));
        assert!(handler.current().key.eq(&DefaultScene));
    }

    // This test documents a likely bug in remove(): it compares raw pointers,
    // so removal by value currently never succeeds. Marked ignored so suite passes.
    #[test]
    // #[ignore]
    fn scene_handler_remove_expected_to_work_but_fails_currently() {
        let mut handler = SceneHandler::new();
        let k = keyA(42);
        handler.add(k, Frame::new(None));
        assert!(
            handler.remove(k).is_some(),
            "remove() failed — pointer comparison bug?"
        );
    }

    // New tests

    #[test]
    fn set_nonexistent_key_does_not_change_current() {
        let mut handler = SceneHandler::new();
        let mut k1 = keyA(1);
        let mut k2 = keyA(2);
        handler.add(k1, Frame::new(None));
        handler.add(k2, Frame::new(None));

        // Move to k2
        handler.set(&mut k2);
        let cur_before = handler.current().key.eq(&k2);
        assert!(cur_before);

        // Try to set to a key not present
        let mut k_missing = keyA(999);
        handler.set(&mut k_missing);

        // Should still be k2
        assert!(handler.current().key.eq(&k2));
    }

    #[test]
    fn remove_nonexistent_key_returns_none_and_keeps_current() {
        let mut handler = SceneHandler::new();
        let k1 = keyA(1);
        handler.add(k1, Frame::new(None));

        let current_is_k1 = handler.current().key.eq(&k1);
        assert!(current_is_k1);

        let k_missing = keyA(999);
        let removed = handler.remove(k_missing);
        assert!(removed.is_none());

        // Current unchanged
        assert!(handler.current().key.eq(&k1));
    }

    #[test]
    fn add_order_preserved() {
        let mut handler = SceneHandler::new();
        let k1 = keyA(1);
        let k2 = keyA(2);
        let k3 = keyA(3);

        handler.add(k1, Frame::new(None));
        handler.add(k2, Frame::new(None));
        handler.add(k3, Frame::new(None));

        // current should be first (k1)
        assert!(handler.current().key.eq(&k1));

        // Verify internal ordering by stepping via set()
        let mut k2m = k2;
        handler.set(&mut k2m);
        assert!(handler.current().key.eq(&k2));

        let mut k3m = k3;
        handler.set(&mut k3m);
        assert!(handler.current().key.eq(&k3));
    }

    #[test]
    fn scene_new_constructs_with_key() {
        let f = Frame::new(None);
        let k = keyA(7);
        let s = Scene::new(k, f.clone());
        assert!(s.key.eq(&k));
    }

    #[test]
    fn remove_current_scene_then_current_index_points_next() {
        // This test ensures removing the current scene allows access to new current
        let mut handler = SceneHandler::new();
        let k1 = keyA(1);
        let k2 = keyA(2);
        handler.add(k1, Frame::new(None));
        handler.add(k2, Frame::new(None));

        // current is k1
        assert!(handler.current().key.eq(&k1));
        // remove k1
        let removed = handler.remove(k1);
        assert!(removed.is_some());

        // current_pos stayed 0, so now should point at former k2
        assert!(handler.current().key.eq(&k2));
    }

    #[test]
    fn remove_scene_after_current_keeps_current_valid() {
        let mut handler = SceneHandler::new();
        let k1 = keyA(1);
        let k2 = keyA(2);
        handler.add(k1, Frame::new(None));
        handler.add(k2, Frame::new(None));

        // stay at k1, remove k2
        let removed = handler.remove(k2);
        assert!(removed.is_some());
        assert!(handler.current().key.eq(&k1));
    }
}
