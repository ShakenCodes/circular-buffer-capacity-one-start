struct CircularBuffer {
    num_elem: usize,
}

impl CircularBuffer {
    pub fn new(_: usize) -> CircularBuffer {
        CircularBuffer { num_elem: 0 }
    }
    pub fn is_empty(&self) -> bool { self.num_elem == 0 }
    pub fn is_full(&self) -> bool { self.num_elem > 0 }
    pub fn put(&mut self, _: i32) -> bool {
        self.num_elem = self.num_elem + 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_capacity_one_when_created_then_is_empty_true_is_full_false() {
        let b = CircularBuffer::new(1);
        assert_eq!(true, b.is_empty());
        assert_eq!(false, b.is_full());
    }
    #[test]
    fn given_capacity_one_when_put_then_return_true_is_empty_false() {
        let mut b = CircularBuffer::new(1);
        assert_eq!(true, b.put(42));
        assert_eq!(false, b.is_empty());
        assert_eq!(true, b.is_full());
    }
}
