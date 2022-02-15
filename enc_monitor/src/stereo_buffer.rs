use std::cmp::min;
use std::collections::vec_deque::Drain;
use std::collections::VecDeque;


pub struct StereoBuffer<S> {
    l: VecDeque<S>,
    r: VecDeque<S>,
}

impl<S> Default for StereoBuffer<S> {
    fn default() -> Self {
        StereoBuffer::new(8192)
    }
}

impl<S> StereoBuffer<S> {
    pub fn new(capacity: usize) -> Self {
        StereoBuffer {
            l: VecDeque::with_capacity(capacity),
            r: VecDeque::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.l.len()
    }

    pub fn enqueue(&mut self, sample: (S, S)) {
        self.l.push_back(sample.0);
        self.r.push_back(sample.1);
    }

    // returns iter_l, iter_r, length of iter
    pub fn dequeue(&mut self, length: usize) -> (Drain<S>, Drain<S>, usize) {
        let ret_length = min(length, self.l.len());
        let iter_l = self.l.drain(..ret_length);
        let iter_r = self.r.drain(..ret_length);
        (iter_l, iter_r, ret_length)
    }

    pub fn cancel(&mut self, length: usize) {
        let _ = self.l.drain(self.l.len() - length..);
        let _ = self.r.drain(self.r.len() - length..);
    }
}

impl StereoBuffer<f32> {
    pub fn enqueue_padding(&mut self, length: usize) {
        for _ in 0..length {
            self.enqueue((0.0, 0.0));
        }
    }
}