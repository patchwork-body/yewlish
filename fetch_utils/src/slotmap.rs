use std::collections::HashMap;

#[derive(Default)]
pub struct SlotMap<T> {
    pub counter: usize,
    pub items: HashMap<usize, T>,
}

impl<T> SlotMap<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            counter: 0,
            items: HashMap::new(),
        }
    }

    #[must_use]
    pub fn first(&self) -> Option<&T> {
        self.items.keys().next().and_then(|id| self.items.get(id))
    }

    pub fn insert(&mut self, item: T) -> usize {
        let id = self.counter;
        self.items.insert(id, item);
        self.counter += 1;
        id
    }

    #[must_use]
    pub fn get(&self, id: usize) -> Option<&T> {
        self.items.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.items.get_mut(&id)
    }

    pub fn remove(&mut self, id: usize) -> Option<T> {
        self.items.remove(&id)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&usize, &T)> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&usize, &mut T)> {
        self.items.iter_mut()
    }
}

impl<T> IntoIterator for SlotMap<T> {
    type Item = (usize, T);
    type IntoIter = std::collections::hash_map::IntoIter<usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a SlotMap<T> {
    type Item = (&'a usize, &'a T);
    type IntoIter = std::collections::hash_map::Iter<'a, usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SlotMap<T> {
    type Item = (&'a usize, &'a mut T);
    type IntoIter = std::collections::hash_map::IterMut<'a, usize, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut()
    }
}
