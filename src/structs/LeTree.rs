enum NodeType {
    Char,
    Star,
    Plus,
    Or,
    Concat
}

struct Node<T> {
    node_type: NodeType,
    value: T,
    right: Option<Node<T>>,
    left: Option<Node<T>>,
}

impl<T> Node<T> {
    pub fn new(node_type: NodeType, value: T, left_child: Node<T>, right_child: Node<T>) -> Self<T> {
        Node::<T> {
            node_type,
            value,
            right: Some(right_child),
            left: Some(left_child)
        }
    }
}

struct BinaryTree<T> {
    root: Node<T>
}

impl<T> BinaryTree<T> {
    pub fn new(root: Node<T>) -> Self<T> {
        BinaryTree::<T> {
            root
        }
    }
}