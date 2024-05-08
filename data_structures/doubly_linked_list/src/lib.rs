use std::cell::RefCell;
use std::rc::{Rc, Weak};

type NodeStrongRef<T> = Rc<RefCell<T>>;
type NodeWeakRef<T> = Weak<RefCell<T>>;

struct Node<T> {
    value: T,
    prev: Option<NodeWeakRef<Node<T>>>,
    next: Option<NodeStrongRef<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }

    fn set_next(&mut self, next: NodeStrongRef<Node<T>>) {
        self.next = Some(next);
    }

    fn set_prev(&mut self, prev: NodeWeakRef<Node<T>>) {
        self.prev = Some(prev);
    }
}

struct DEQ<T> {
    head: Option<NodeStrongRef<Node<T>>>,
    tail: Option<NodeWeakRef<Node<T>>>,
    size: i32,
}

impl<T> DEQ<T> {
    fn new() -> Self {
        DEQ {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn peek_front(&self) -> Option<T>
    where
        T: Clone,
    {
        match self.head.as_ref() {
            Some(node) => Some(node.borrow().value.clone()),
            None => None,
        }
    }

    fn peek_back(&self) -> Option<T>
    where
        T: Clone,
    {
        if let Some(weak_tail) = self.tail.as_ref() {
            if let Some(node) = weak_tail.upgrade() {
                Some(node.borrow().value.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));

        match self.head.take() {
            Some(head) => {
                head.borrow_mut().set_prev(Rc::downgrade(&new_node));
                // Head becomes tail if it doesn't point to next
                if head.borrow().next.is_none() {
                    self.tail = Some(Rc::downgrade(&head));
                }
                new_node.borrow_mut().set_next(head);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }
    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.tail.take() {
            Some(tail) => {
                if let Some(node) = tail.upgrade() {
                    new_node.borrow_mut().set_prev(Rc::downgrade(&node));
                    self.tail = Some(Rc::downgrade(&new_node));
                    node.borrow_mut().set_next(new_node);
                }
            }
            None => {
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }

        self.size += 1;
    }
    fn pop_front(&mut self) -> Option<T>
    where
        T: Clone,
    {
        if let Some(head) = self.head.take() {
            let out = head.borrow().value.clone();
            self.head = head.borrow().next.clone();
            self.size -= 1;
            Some(out)
        } else {
            None
        }
    }

    fn pop_back(&mut self) -> Option<T>
    where
        T: Clone,
    {
        let tail = self.tail.take()?.upgrade()?;
        let out = tail.borrow().value.clone();

        self.size -= 1;

        // deq has one list
        if Rc::ptr_eq(self.head.as_ref().unwrap(), &tail) {
            self.head = None;
            self.tail = None;
            return Some(tail.borrow().value.clone());
        }

        let new_tail = tail.borrow().prev.as_ref()?.upgrade()?;

        new_tail.borrow_mut().next = None;
        self.tail = Some(Rc::downgrade(&new_tail));

        Some(out)
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_should_push_front_empty_deq_correctly() {
        let mut deq: DEQ<i32> = DEQ::new();
        deq.push_front(1);

        assert_eq!(deq.peek_front().unwrap(), 1);
        assert_eq!(deq.size, 1);
    }

    #[test]
    fn it_should_push_front_correctly() {
        let mut deq = DEQ::<i32>::new();

        deq.push_front(3);
        deq.push_front(2);
        deq.push_front(1);

        assert_eq!(deq.size, 3);
        assert_eq!(deq.peek_front().unwrap(), 1);
    }

    #[test]
    fn it_should_push_back_empty_deq_correctly() {
        let mut deq = DEQ::<i32>::new();

        deq.push_back(1);

        assert_eq!(deq.peek_front().unwrap(), 1);
        assert_eq!(deq.peek_back().unwrap(), 1);
        assert_eq!(deq.size, 1);
    }

    #[test]
    fn it_should_push_back_correctly() {
        let mut deq = DEQ::<i32>::new();

        deq.push_back(1);
        deq.push_back(2);
        deq.push_back(3);

        assert_eq!(deq.peek_front().unwrap(), 1);
        assert_eq!(deq.peek_back().unwrap(), 3);
        assert_eq!(deq.size, 3);
    }

    #[test]
    fn it_should_pop_front_correctly() {
        let mut deq = DEQ::<i32>::new();

        deq.push_back(1);
        deq.push_back(2);
        deq.push_back(3);

        let head = deq.pop_front();

        assert_eq!(head.unwrap(), 1);
    }

    #[test]
    fn it_should_pop_back_correctly() {
        let mut deq = DEQ::<i32>::new();

        deq.push_back(1);
        deq.push_back(2);
        deq.push_back(3);

        let v = deq.pop_back();

        assert_eq!(v.unwrap(), 3);
        assert_eq!(deq.size, 2);
    }

    #[test]
    fn it_should_be_able_to_work_with_many_operations() {
        let mut deq = DEQ::<i32>::new();

        deq.push_back(1);
        deq.push_back(2);
        deq.push_back(3);
        deq.push_front(0);
        deq.push_front(-1);
        deq.push_front(-2);
        deq.push_front(-3);

        assert_eq!(deq.peek_front().unwrap(), -3);
        assert_eq!(deq.peek_back().unwrap(), 3);

        for _ in 0..10 {
            deq.pop_front();
        }

        assert_eq!(deq.peek_front(), None);
        assert_eq!(deq.size, 0);
    }
}
