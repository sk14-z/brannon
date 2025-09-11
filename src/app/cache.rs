pub struct AppCache<T> {
    keys: Vec<String>,
    buf: Vec<Box<T>>,
}

impl<T> AppCache<T> {
    pub(crate) fn new() -> Self {
        AppCache::<T> {
            keys: Vec::new(),
            buf: Vec::new(),
        }
    }

    pub fn add(&mut self, name: &'static str, value: T) {
        self.keys.push(String::from(name));
        self.buf.push(Box::new(value));
    }

    pub fn remove(&mut self, name: &'static str) -> Option<Box<T>> {
        if let Some(i) = self.keys.iter().position(|w| w == name) {
            self.keys.remove(i);
            Some(self.buf.remove(i))
        } else {
            None
        }
    }

    pub fn get(&mut self, name: &'static str) -> Option<&Box<T>> {
        if let Some(i) = self.keys.iter().position(|w| w == name) {
            self.buf.get(i)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, name: &'static str) -> Option<&mut Box<T>> {
        if let Some(i) = self.keys.iter().position(|w| w == name) {
            self.buf.get_mut(i)
        } else {
            None
        }
    }
}
