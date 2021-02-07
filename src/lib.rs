struct CircularBuffer {}

impl CircularBuffer {
    pub fn new(_: usize) -> CircularBuffer {
        CircularBuffer {}
    }
    pub fn is_empty(&self) -> bool { true }
    pub fn is_full(&self) -> bool { false }
    pub fn put(&self, _: i32) -> bool { true }
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
    fn given_capacity_one_when_put_then_return_true() {
        let b = CircularBuffer::new(1);
        assert_eq!(true, b.put(42));
    }
}
