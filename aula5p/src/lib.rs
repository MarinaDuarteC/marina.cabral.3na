struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}


struct BST {
    root: Option<Box<Node>>,
}

impl BST {
   
    fn new() -> Self {
        BST { root: None }
    }
    
    
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        self.root = Self::insert_recursive(self.root.take(), value);
    }
    
    
    fn insert_recursive(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match node {
            Some(mut n) => {
                if value < n.value {
                    n.left = Self::insert_recursive(n.left.take(), value);
                } else {
                    n.right = Self::insert_recursive(n.right.take(), value);
                }
                Some(n)
            },
            None => Some(Box::new(Node::new(value))),
        }
    }
    
    
    fn search(&self, value: i32) -> bool {
        Self::search_recursive(&self.root, value)
    }

    
    fn search_recursive(node: &Option<Box<Node>>, value: i32) -> bool {
        match node {
            Some(n) => {
                if value == n.value {
                    true
                } else if value < n.value {
                    Self::search_recursive(&n.left, value)
                } else {
                    Self::search_recursive(&n.right, value)
                }
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod bst_tests {
    use crate::BST;
    
    #[test]
    fn test_bst_new_and_empty() {
        
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        
        let mut bst = BST::new();
        
        
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        
        assert!(!bst.search(20));
        
        
        assert!(!bst.is_empty());
    }
}