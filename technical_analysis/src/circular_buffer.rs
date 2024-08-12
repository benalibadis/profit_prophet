#[derive(Debug, Clone)]
pub struct CircularBuffer {
    buffer: Vec<f64>,
    index: usize,
    full: bool,
    capacity: usize,
    capacity_mask: usize,
}

impl CircularBuffer {
    #[inline(always)]
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![0.0; capacity],
            index: 0,
            full: false,
            capacity,
            capacity_mask: capacity - 1,
        }
    }

    #[cfg(not(feature = "unsafe"))]
    #[inline(always)]
    pub fn push(&mut self, value: f64) -> f64 {
        let old_value = std::mem::replace(&mut self.buffer[self.index], value);

        self.index = (self.index + 1) & self.capacity_mask;

        if self.index == 0 {
            self.full = true;
        }

        old_value
    }

    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub fn push(&mut self, value: f64) -> f64 {
        let old_value = unsafe {
            std::mem::replace(self.buffer.get_unchecked_mut(self.index), value)
        };

        self.index = (self.index + 1) & self.capacity_mask;

        if self.index == 0 {
            self.full = true;
        }

        old_value
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        if self.full {
            self.capacity
        } else {
            self.index
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.full
    }

    #[cfg(not(feature = "unsafe"))]
    #[inline(always)]
    pub fn clear(&mut self) {
        for elem in &mut self.buffer {
            *elem = 0.0;
        }
        self.index = 0;
        self.full = false;
    }

    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..self.capacity {
                *self.buffer.get_unchecked_mut(i) = 0.0;
            }
        }
        self.index = 0;
        self.full = false;
    }
}
