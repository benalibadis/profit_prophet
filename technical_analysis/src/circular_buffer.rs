use std::iter::FusedIterator;
use crate::IndicatorValue;

#[derive(Debug, Clone)]
pub struct CircularBuffer {
    buffer: Vec<IndicatorValue>,
    index: usize,
    full: bool,
    capacity: usize,
    capacity_mask: usize,
}

impl CircularBuffer {
    #[inline(always)]
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![0.0.into(); capacity],
            index: 0,
            full: false,
            capacity,
            capacity_mask: capacity.saturating_sub(1), // Use saturating_sub for safety.
        }
    }

    #[cfg(not(feature = "unsafe"))]
    #[inline(always)]
    pub fn push(&mut self, value: IndicatorValue) -> IndicatorValue {
        let old_value = std::mem::replace(&mut self.buffer[self.index], value);

        self.index = (self.index + 1) & self.capacity_mask;

        if self.index == 0 {
            self.full = true;
        }

        old_value
    }

    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub fn push(&mut self, value: IndicatorValue) -> IndicatorValue {
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
            *elem = 0.0.into();
        }
        self.index = 0;
        self.full = false;
    }

    #[cfg(feature = "unsafe")]
    #[inline(always)]
    pub fn clear(&mut self) {
        unsafe {
            for i in 0..self.capacity {
                *self.buffer.get_unchecked_mut(i) = 0.0.into();
            }
        }
        self.index = 0;
        self.full = false;
    }

    #[inline(always)]
    pub fn iter(&self) -> CircularBufferIterator {
        CircularBufferIterator::new(self)
    }

    #[inline(always)]
    pub fn iter_reversed(&self) -> ReversedCircularBufferIterator {
        ReversedCircularBufferIterator::new(self)
    }
}

pub struct ReversedCircularBufferIterator<'a> {
    buffer: &'a CircularBuffer,
    index: usize,
    remaining: usize,
}

impl<'a> ReversedCircularBufferIterator<'a> {
    #[inline(always)]
    pub fn new(buffer: &'a CircularBuffer) -> Self {
        let remaining = buffer.len();
        let index = if buffer.is_full() {
            (buffer.index.wrapping_add(buffer.capacity()).wrapping_sub(1)) & buffer.capacity_mask
        } else if buffer.index == 0 {
            0
        } else {
            buffer.index.wrapping_sub(1)
        };

        ReversedCircularBufferIterator {
            buffer,
            index,
            remaining,
        }
    }
}

impl<'a> Iterator for ReversedCircularBufferIterator<'a> {
    type Item = IndicatorValue;

    #[inline(always)]
    #[cfg(feature = "unsafe")]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = unsafe { *self.buffer.buffer.get_unchecked(self.index) };
        self.index = if self.index == 0 {
            self.buffer.capacity_mask
        } else {
            self.index.wrapping_sub(1)
        };
        self.remaining -= 1;

        Some(value)
    }

    #[inline(always)]
    #[cfg(not(feature = "unsafe"))]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = self.buffer.buffer[self.index];
        self.index = if self.index == 0 {
            self.buffer.capacity_mask
        } else {
            self.index.wrapping_sub(1)
        };
        self.remaining -= 1;

        Some(value)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> ExactSizeIterator for ReversedCircularBufferIterator<'a> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<'a> FusedIterator for ReversedCircularBufferIterator<'a> {}

pub struct CircularBufferIterator<'a> {
    buffer: &'a CircularBuffer,
    index: usize,
    remaining: usize,
}

impl<'a> CircularBufferIterator<'a> {
    #[inline(always)]
    pub fn new(buffer: &'a CircularBuffer) -> Self {
        let remaining = buffer.len();
        let index = if buffer.is_full() {
            buffer.index
        } else {
            0
        };

        CircularBufferIterator {
            buffer,
            index,
            remaining,
        }
    }
}

impl<'a> Iterator for CircularBufferIterator<'a> {
    type Item = IndicatorValue;

    #[inline(always)]
    #[cfg(feature = "unsafe")]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = unsafe { *self.buffer.buffer.get_unchecked(self.index) };
        self.index = (self.index + 1) & self.buffer.capacity_mask;
        self.remaining -= 1;

        Some(value)
    }

    #[inline(always)]
    #[cfg(not(feature = "unsafe"))]
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        let value = self.buffer.buffer[self.index];
        self.index = (self.index + 1) & self.buffer.capacity_mask;
        self.remaining -= 1;

        Some(value)
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'a> ExactSizeIterator for CircularBufferIterator<'a> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.remaining
    }
}

impl<'a> FusedIterator for CircularBufferIterator<'a> {}
