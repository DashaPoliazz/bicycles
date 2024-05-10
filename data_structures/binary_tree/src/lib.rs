struct Node<T>
where
    T: PartialOrd,
{
    value: T,
    left: BinarySearchTree<T>,
    right: BinarySearchTree<T>,
}

impl<T> Node<T>
where
    T: PartialOrd,
{
    fn new(value: T) -> Self {
        Node {
            value,
            left: BinarySearchTree(None),
            right: BinarySearchTree(None),
        }
    }
}

struct BinarySearchTree<T: PartialOrd>(Option<Box<Node<T>>>);
impl<T> BinarySearchTree<T>
where
    T: PartialOrd,
{
    fn new() -> Self {
        BinarySearchTree(None)
    }

    fn insert(&mut self, value: T) {
        match self.0 {
            Some(ref mut node) => {
                if value < node.value {
                    node.left.insert(value)
                } else if value > node.value {
                    node.right.insert(value)
                } else {
                    return;
                }
            }
            None => self.0 = Some(Box::new(Node::new(value))),
        }
    }

    pub fn inorder(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.inorder_traverse(vec![])
    }

    fn inorder_traverse(&self, mut result: Vec<T>) -> Vec<T>
    where
        T: Clone,
    {
        if let Some(ref node) = self.0 {
            result = node.left.inorder_traverse(result);
            result.push(node.value.clone());
            result = node.right.inorder_traverse(result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bst = BinarySearchTree::<i32>::new();

        bst.insert(10);
        bst.insert(5);
        bst.insert(15);

        let mut inorder_traversal = bst.inorder();
        assert_eq!(vec![5, 10, 15], inorder_traversal);
    }
}
