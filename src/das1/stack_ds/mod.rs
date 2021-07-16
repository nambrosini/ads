struct Stack<T> {
    top: i64,
    stack: Vec<T>,
    capacity: i64
}

impl<T> Stack<T> {
    fn new(size: i64) -> Self {
        Self {
            top: -1,
            stack: Vec::new(),
            capacity: size
        }
    }

    fn push(&mut self, value: T) {
        if self.is_full() {
            return;
        }
        self.top += 1;
        self.stack.push(value)
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.top -= 1;
        self.stack.pop()
    }

    fn is_empty(&self) -> bool {
        self.top == -1
    }

    fn is_full(&self) -> bool {
        self.top + 1 == self.capacity
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.stack.last()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        let nico = "Nico";
        let mut reverse = String::new();

        let mut stack = Stack::<char>::new(4);

        for c in nico.chars() {
            stack.push(c);
        }

        while let Some(c) = stack.pop() {
            reverse.push(c);
        }

        assert_eq!(reverse, "ociN");
    }
}