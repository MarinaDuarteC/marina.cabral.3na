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

// Estrutura para a Árvore Binária de Busca (BST)
struct BST {
    root: Option<Box<Node>>,
}

impl BST {
    // Criar uma nova árvore vazia
    fn new() -> Self {
        BST { root: None }
    }
    
    // Verificar se a árvore está vazia
    fn is_empty(&self) -> bool {
        self.root.is_none()
    }
    
    // Inserir um valor na árvore
    fn insert(&mut self, value: i32) {
        self.root = Self::insert_recursive(self.root.take(), value);
    }
    
    // Função recursiva para inserir um valor
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
    
    // Buscar um valor na árvore
    fn search(&self, value: i32) -> bool {
        Self::search_recursive(&self.root, value)
    }

    // Função recursiva para buscar um valor
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
        // Teste 1: Criar uma nova árvore e verificar se está vazia
        let bst = BST::new();
        assert!(bst.is_empty());
    }
    
    #[test]
    fn test_bst_insert_and_search() {
        // Teste 2: Inserir elementos e verificar se estão na árvore
        let mut bst = BST::new();
        
        // Inserir alguns valores
        bst.insert(10);
        bst.insert(5);
        bst.insert(15);
        
        // Verificar se os valores inseridos estão na árvore
        assert!(bst.search(10));
        assert!(bst.search(5));
        assert!(bst.search(15));
        
        // Verificar que um valor não inserido não está na árvore
        assert!(!bst.search(20));
        
        // A árvore não deve mais estar vazia
        assert!(!bst.is_empty());
    }
}