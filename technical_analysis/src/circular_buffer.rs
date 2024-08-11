#[derive(Debug, Clone)]
pub struct CircularBuffer {
    buffer: Vec<f64>,
    index: usize,
    full: bool,
}

impl CircularBuffer {
    #[inline(always)]
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![0.0; capacity],
            index: 0,
            full: false,
        }
    }

    #[inline(always)]
    pub fn push(&mut self, value: f64) -> f64 {
        let old_value = unsafe { *self.buffer.get_unchecked(self.index) };
        unsafe {
            *self.buffer.get_unchecked_mut(self.index) = value;
        }
        self.index = self.index.wrapping_add(1);
        if self.index == self.buffer.len() {
            self.index = 0;
            self.full = true;
        }
        old_value
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        if self.full {
            self.buffer.len()
        } else {
            self.index
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.full
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        for elem in &mut self.buffer {
            *elem = 0.0;
        }
        self.index = 0;
        self.full = false;
    }
}