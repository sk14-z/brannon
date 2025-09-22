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

    pub fn get(&mut self, name: &'static str) -> Option<&T> {
        if let Some(i) = self.keys.iter().position(|w| w == name) {
            Some(self.buf.get(i)?)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, name: &'static str) -> Option<&mut T> {
        if let Some(i) = self.keys.iter().position(|w| w == name) {
            Some(self.buf.get_mut(i)?)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &'static str, value: T) -> Option<()> {
        if let Some(i) = self.keys.iter().position(|w| w == name) {
            self.buf[i] = Box::new(value);
            Some(())
        } else {
            None
        }
    }
}

impl<T> AppCache<T>
where
    T: Clone + Copy,
{
    pub fn value(&self, name: &'static str) -> Option<T> {
        if let Some(i) = self.keys.iter().position(|w| w == name) {
            Some(*self.buf.get(i)?.clone())
        } else {
            None
        }
    }
}
