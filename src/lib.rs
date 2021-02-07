struct CircularBuffer {
    capacity: usize,
    num_elem: usize,
    at_in: usize,
    at_out: usize,
    elems: Vec<i32>,
}

impl CircularBuffer {
    pub fn new(c: usize) -> CircularBuffer {
        CircularBuffer {
            capacity: c,
            num_elem: 0,
            at_in: 0,
            at_out: 0,
            elems: vec![i32::MIN; c],
        }
    }
    pub fn is_empty(&self) -> bool { self.num_elem == 0 }
    pub fn is_full(&self) -> bool { self.num_elem >= self.capacity }
    pub fn put(&mut self, v: i32) -> bool {
        if self.is_full() { return false }
        self.num_elem = self.num_elem + 1;
        self.elems[self.at_in] = v;
        self.at_in = CircularBuffer::increment_and_clip(self.at_in, self.capacity);
        true
    }
    pub fn get(&mut self) -> i32 {
        if self.is_empty() { return i32::MIN }
        self.num_elem = self.num_elem - 1;
        let v = self.elems[self.at_out];
        self.at_out = CircularBuffer::increment_and_clip(self.at_out, self.capacity);
        v
    }
    fn increment_and_clip(n: usize, c: usize) -> usize {
        if (n + 1) >= c { return 0 }
        n + 1
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
    #[test]
    fn given_capacity_one_with_one_put_when_put_then_return_false() {
        let mut b = CircularBuffer::new(1);
        assert!(b.put(42));
        assert_eq!(false, b.put(44));
    }
    #[test]
    fn given_capacity_twe_with_two_puts_when_put_then_return_false() {
        let mut b = CircularBuffer::new(2);
        assert!(b.put(42));
        assert!(b.put(44));
        assert_eq!(false, b.put(46));
    }
    #[test]
    fn given_capacity_twe_with_two_puts_if_empty_false() {
        let mut b = CircularBuffer::new(2);
        assert!(b.put(42));
        assert!(b.put(44));
        assert_eq!(false, b.is_empty());
        assert_eq!(true, b.is_full());
    }
    #[test]
    fn given_capacity_twe_with_two_puts_when_get_twice_then_return_put_values() {
        let mut b = CircularBuffer::new(2);
        let v2 = 99;
        assert!(b.put(42));
        assert!(b.put(v2));
        b.get();
        assert!(b.put(44));
        assert_eq!(v2, b.get());
    }
    #[test]
    fn given_capacity_twe_with_two_puts_two_gets_two_puts_when_get_twice_then_return_put_values() {
        let mut b = CircularBuffer::new(2);
        assert!(b.put(42));
        assert!(b.put(44));
        b.get();
        b.get();
        let v1 = -11;
        let v2 = 99;
        assert!(b.put(v1));
        assert!(b.put(v2));
        assert_eq!(v1, b.get());
        assert_eq!(v2, b.get());
    }
}
