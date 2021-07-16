pub struct Queue<T>
where
    T: Copy,
{
    rear: i64,
    front: i64,
    capacity: usize,
    queue: Vec<T>,
}

impl<T> Queue<T>
where
    T: Copy,
{
    fn new(size: usize) -> Self {
        Self {
            rear: -1,
            front: -1,
            capacity: size,
            queue: Vec::with_capacity(size),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        if self.is_full() {
            return;
        }

        if self.front == -1 {
            self.front = 0;
        }

        self.rear += 1;
        if self.queue.len() == self.capacity {
            self.queue[self.rear as usize] = value;
        } else {
            self.queue.push(value);
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let element = self.queue[self.front as usize];

        if self.front as usize == self.capacity - 1 {
            self.front = -1;
            self.rear = -1;
        } else {
            self.front += 1;
        }

        Some(element)
    }

    pub fn is_empty(&self) -> bool {
        self.rear == -1
    }

    pub fn is_full(&self) -> bool {
        self.rear as usize == self.capacity - 1
    }

    pub fn peek(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        Some(self.queue[self.front as usize])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let mut queue: Queue<i32> = Queue::new(3);

        assert!(queue.is_empty());
        assert!(!queue.is_full());
        assert!(queue.peek().is_none());

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert!(!queue.is_empty());
        assert!(queue.is_full());

        assert_eq!(queue.peek(), Some(1));

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);

        assert!(queue.is_empty());

        queue.enqueue(1);

        assert_eq!(queue.peek(), Some(1));
    }
}