struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, val: T) {
        let new_next = self.0.take();
        self.0 = Some((val, Box::new(LinkedList(new_next))));
    }

    pub fn push_back(&mut self, val: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(val),
            None => self.push_front(val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_front() {
        let mut ll = LinkedList::new();
        ll.push_front(2);
        ll.push_front(1);

        let head_value = ll.0.unwrap();
        assert_eq!(head_value.0, 1);
    }

    #[test]
    fn push_back() {
        let mut ll = LinkedList::new();
        ll.push_back(1);
        ll.push_back(2);

        let head_value = ll.0.unwrap();
        assert_eq!(head_value.0, 1);
    }
}
