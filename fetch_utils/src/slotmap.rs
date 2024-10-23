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
}
