#[derive(Debug, Clone)]
pub struct ObjectPool<T: Default> {
    pool: Vec<T>,
    free: Vec<usize>,
}

impl<T: Default> ObjectPool<T> {
    pub fn new(reserve_size: usize) -> Self {
        ObjectPool {
            pool: Vec::with_capacity(reserve_size),
            free: Vec::new(),
        }
    }

    pub fn allocate(&mut self) -> usize {
        match self.free.pop() {
            Some(idx) => idx,
            None => {
                self.pool.push(T::default());
                self.pool.len() - 1
            }
        }
    }

    pub fn get(&self, idx: usize) -> &T {
        &self.pool[idx]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut T {
        &mut self.pool[idx]
    }

    pub fn free(&mut self, idx: usize) {
        self.free.push(idx);
    }
}
