#![allow(dead_code)]
struct MyCircularQueue {
    head: usize,
    tail: usize,
    values: Vec<i32>,
    length: usize,
    count: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            head: 0,
            tail: 0,
            values: vec![0; k as usize],
            length: k as usize,
            count: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.count == 0 {
            self.values[self.head] = value;
            self.tail = self.head;
            self.count += 1;
            return true;
        }
        if self.is_full() {
            false
        } else {
            let new_tail = (self.tail + 1) % self.length;
            self.count += 1;
            self.values[new_tail] = value;
            self.tail = new_tail;
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.count == 0 {
            return false;
        }
        self.head = (self.head + 1) % self.length;
        self.count -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[self.head]
        }
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            -1
        } else {
            self.values[self.tail]
        }
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn is_full(&self) -> bool {
        self.count == self.length
    }
}

/**
 * Your MyCircularQueue object will be instantiated and called as such:
 * let obj = MyCircularQueue::new(k);
 * let ret_1: bool = obj.en_queue(value);
 * let ret_2: bool = obj.de_queue();
 * let ret_3: i32 = obj.front();
 * let ret_4: i32 = obj.rear();
 * let ret_5: bool = obj.is_empty();
 * let ret_6: bool = obj.is_full();
 */

#[cfg(test)]
mod tests_622 {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = MyCircularQueue::new(3);
        assert!(queue.en_queue(1));
        assert!(queue.en_queue(2));
        assert!(queue.en_queue(3));
        assert!(!queue.en_queue(4));
        assert_eq!(queue.rear(), 3);
        assert!(queue.is_full());
        assert!(queue.de_queue());
        assert!(queue.en_queue(4));
        assert_eq!(queue.rear(), 4);
    }
}
