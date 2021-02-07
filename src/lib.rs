struct CircularBuffer {}

impl CircularBuffer {
    pub fn new(_: usize) -> CircularBuffer {
        CircularBuffer {}
    }
    pub fn is_empty(&self) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_capacity_one_when_created_then_is_empty_true() {
        let b = CircularBuffer::new(1);
        assert_eq!(true, b.is_empty());
    }
}
