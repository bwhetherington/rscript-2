use std::collections::HashMap;

use super::lexer::Token;

struct PrefixNode<T> {
    value: Option<T>,
    children: HashMap<char, PrefixNode<T>>,
}

impl<T> PrefixNode<T> {
    pub fn root() -> PrefixNode<T> {
        PrefixNode {
            value: None,
            children: HashMap::new(),
        }
    }

    pub fn new(value: Option<T>) -> PrefixNode<T> {
        PrefixNode {
            value,
            children: HashMap::new(),
        }
    }

    pub fn find(&self, prefix: &str, depth: usize) -> Option<(&PrefixNode<T>, usize)> {
        match prefix.chars().next() {
            None => return Some((self, depth)),
            Some(first_char) => {
                let child = self.children.get(&first_char)?;
                let slice = &prefix[1..];
                child.find(slice, depth + 1)
            }
        }
    }

    pub fn find_or_create(&mut self, prefix: &str) -> &mut PrefixNode<T> {
        match prefix.chars().next() {
            None => self,
            Some(first_char) => {
                let child_ref = self
                    .children
                    .entry(first_char)
                    .or_insert_with(|| PrefixNode::new(None));
                child_ref.find_or_create(&prefix[1..])
            }
        }
    }
}

pub struct PrefixTree<T> {
    root: PrefixNode<T>,
}

impl<T> PrefixTree<T> {
    pub fn new() -> PrefixTree<T> {
        PrefixTree {
            root: PrefixNode::root(),
        }
    }

    pub fn insert(&mut self, prefix: &str, value: T) {
        let node = self.root.find_or_create(prefix);
        node.value = Some(value);
    }

    pub fn find(&self, prefix: &str) -> Option<(&PrefixNode<T>, usize)> {
        self.root.find(prefix, 0)
    }
}
