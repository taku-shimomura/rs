use std::cell::RefCell;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct Queue<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, val: T) {
        todo!("未実装");
    }

    pub fn dequeue(&mut self) {
        todo!("未実装");
    }

    pub fn front(&mut self) -> Option<T> {
        todo!("未実装");
    }

    pub fn back(&mut self) -> Option<T> {
        todo!("未実装");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue() {
        let mut q = Queue::new();
        for i in 1..=5 {
            q.enqueue(i);
        }
        assert_eq!(q.front(), Some(1));
        assert_eq!(q.back(), Some(5));
        for _ in 0..3 {
            q.dequeue();
        }
        assert_eq!(q.front(), Some(4));
        assert_eq!(q.back(), Some(5));
        for _ in 0..2 {
            q.dequeue();
        }
        assert_eq!(q.front(), None);
        assert_eq!(q.back(), None);
    }
}
