
#[derive(Clone, Debug, Default)]
pub struct HoleVec<T> {
    inner: Vec<Option<T>>,
    first_free: usize,
    count: usize,
}

impl<T> HoleVec<T> {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            first_free: 0,
            count: 0,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            first_free: 0,
            count: 0,
        }
    }
    pub fn push(&mut self, e: T) -> usize {
        while self.first_free < self.inner.len() && self.inner[self.first_free].is_none() {
            self.first_free += 1;
        }
        if self.first_free >= self.inner.len() {
            self.inner.push(Some(e));
            self.first_free = self.inner.len();
        } else {
            self.inner[self.first_free] = Some(e);
            self.first_free += 1;
        }

        self.count += 1;

        self.first_free - 1
    }
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.first_free {
            self.first_free = index;
        }
        if self.inner[index].is_some() {
            self.count -= 1;
        }
        self.inner[index].take()
    }
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        for (i, e) in self
            .inner
            .iter_mut()
            .enumerate()
            .filter(|(_, e)| e.is_some())
        {
            if !f(e.as_ref().unwrap()) {
                e.take();
                self.count -= 1;
                if i < self.first_free {
                    self.first_free = i;
                }
            }
        }
    }
    pub fn clear(&mut self) {
        self.inner.clear();
        self.first_free = 0;
        self.count = 0;
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.inner.get(index).and_then(|e| e.as_ref())
    }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.inner.get_mut(index).and_then(|e| e.as_mut())
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter().filter_map(|e| e.as_ref())
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.inner.iter_mut().filter_map(|e| e.as_mut())
    }
    pub fn contains(&self, x: &T) -> bool
    where
        T: PartialEq<T>,
    {
        for e in self.inner.iter().filter_map(|e| e.as_ref()) {
            if x == e {
                return true;
            }
        }
        false
    }

    // replicas
    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional)
    }
    pub fn reserve_exact(&mut self, additional: usize) {
        self.inner.reserve_exact(additional)
    }
    pub fn shrink_to_fit(&mut self) {
        self.inner.shrink_to_fit()
    }
}

use std::ops::*;
