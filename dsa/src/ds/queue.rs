use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("val", &self.val)
            .field(
                "next",
                &if self.next.is_some() {
                    "Some(...)"
                } else {
                    "None"
                },
            )
            .field(
                "prev",
                &if self.prev.is_some() {
                    "Some(...)"
                } else {
                    "None"
                },
            )
            .finish()
    }
}

#[derive(Debug)]
pub struct Queue<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T: Clone> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, val: T) {
        let new_node = Rc::new(RefCell::new(Node::new(val)));
        if self.tail.is_none() {
            self.head = Some(new_node.clone());
            self.tail = Some(new_node);
        } else {
            let old_tail = self.tail.take().unwrap();
            old_tail.borrow_mut().next = Some(new_node.clone());
            new_node.borrow_mut().prev = Some(old_tail);
            self.tail = Some(new_node);
        }
    }

    pub fn dequeue(&mut self) {
        if let Some(head) = self.head.take() {
            let mut head_borrow = head.borrow_mut();
            if let Some(next) = head_borrow.next.take() {
                next.borrow_mut().prev = None;
                self.head = Some(next);
            } else {
                self.tail = None;
            }
        }
    }

    pub fn front(&self) -> Option<T> {
        self.head.as_ref().map(|node| node.borrow().val.clone())
    }

    pub fn back(&self) -> Option<T> {
        self.tail.as_ref().map(|node| node.borrow().val.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue() {
        let mut q = Queue::default();
        for i in 1..=5 {
            q.enqueue(i);
        }
        println!("{:?}", q);
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
