pub struct Cache<T> {
    keys: Vec<String>,
    buf: Vec<Box<T>>,
}

impl<T> Cache<T> {
    pub(crate) fn new() -> Self {
        Cache::<T> {
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

impl<T> Cache<T>
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

#[cfg(test)]
mod tests {
    use super::Cache;

    #[test]
    fn test_add_and_get() {
        let mut c = Cache::<i32>::new();
        c.add("a", 10);
        c.add("b", 20);
        assert_eq!(c.get("a"), Some(&10));
        assert_eq!(c.get("b"), Some(&20));
        assert_eq!(c.get("c"), None);
    }

    #[test]
    fn test_get_mut_and_modify() {
        let mut c = Cache::<i32>::new();
        c.add("x", 1);
        {
            let v = c.get_mut("x").unwrap();
            *v = 5;
        }
        assert_eq!(c.get("x"), Some(&5));
    }

    #[test]
    fn test_set_existing() {
        let mut c = Cache::<i32>::new();
        c.add("k", 7);
        assert!(c.set("k", 9).is_some());
        assert_eq!(c.get("k"), Some(&9));
    }

    #[test]
    fn test_set_missing() {
        let mut c = Cache::<i32>::new();
        assert!(c.set("none", 1).is_none());
    }

    #[test]
    fn test_remove() {
        let mut c = Cache::<i32>::new();
        c.add("a", 1);
        c.add("b", 2);
        let removed = c.remove("a");
        assert!(removed.is_some());
        assert_eq!(c.get("a"), None);
        assert_eq!(c.get("b"), Some(&2));
        assert!(c.remove("missing").is_none());
    }

    #[test]
    fn test_value_copy() {
        let mut c = Cache::<i32>::new();
        c.add("n", 42);
        assert_eq!(c.value("n"), Some(42));
        assert_eq!(c.value("missing"), None);
    }

    #[test]
    fn test_multiple_operations_sequence() {
        let mut c = Cache::<i32>::new();
        c.add("a", 1);
        c.add("b", 2);
        c.add("c", 3);
        assert_eq!(c.value("b"), Some(2));
        c.set("b", 20);
        assert_eq!(c.value("b"), Some(20));
        c.remove("a");
        assert_eq!(c.get("a"), None);
        assert_eq!(c.value("c"), Some(3));
    }
}
