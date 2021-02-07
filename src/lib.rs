struct CircularBuffer {
    num_elem: usize,
    elem: i32,
}

impl CircularBuffer {
    pub fn new(_: usize) -> CircularBuffer {
        CircularBuffer { num_elem: 0, elem: i32::MIN }
    }
    pub fn is_empty(&self) -> bool { self.num_elem == 0 }
    pub fn is_full(&self) -> bool { self.num_elem > 0 }
    pub fn put(&mut self, v: i32) -> bool {
        self.num_elem = self.num_elem + 1;
        self.elem = v;
        true
    }
    pub fn get(&mut self) -> i32 {
        if self.num_elem == 0 { return i32::MIN }
        self.num_elem = self.num_elem - 1;
        self.elem
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
    fn given_capacity_one_when_put_then_return_true_is_empty_false_is_full_true() {
        let mut b = CircularBuffer::new(1);
        assert_eq!(true, b.put(42));
        assert_eq!(false, b.is_empty());
        assert_eq!(true, b.is_full());
    }
    #[test]
    fn given_capacity_one_when_get_then_return_min_int() {
        let mut b = CircularBuffer::new(1);
        assert_eq!(i32::MIN, b.get());
    }
    #[test]
    fn given_capacity_one_with_one_put_when_get_then_return_value_put() {
        let mut b = CircularBuffer::new(1);
        let v = 99;
        assert!(b.put(v));
        assert_eq!(v, b.get());
    }
    #[test]
    fn given_capacity_one_with_one_put_when_get_then_is_empty_true_is_full_false() {
        let mut b = CircularBuffer::new(1);
        assert!(b.put(42));
        b.get();
        assert_eq!(true, b.is_empty());
        assert_eq!(false, b.is_full());
    }
    #[test]
    fn given_capacity_one_with_one_put_and_get_when_get_then_return_int_min() {
        let mut b = CircularBuffer::new(1);
        assert!(b.put(42));
        b.get();
        assert_eq!(i32::MIN, b.get());
    }
}
