use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct PrefixNode<T> {
    value: Option<T>,
    children: HashMap<char, PrefixNode<T>>,
}

impl<T> PrefixNode<T> {
    fn root() -> PrefixNode<T> {
        PrefixNode {
            value: None,
            children: HashMap::new(),
        }
    }

    fn new(value: Option<T>) -> PrefixNode<T> {
        PrefixNode {
            value,
            children: HashMap::new(),
        }
    }

    fn find(&self, prefix: &str) -> Option<&PrefixNode<T>> {
        match prefix.chars().next() {
            None => return Some(self),
            Some(first_char) => {
                let child = self.children.get(&first_char)?;
                let slice = &prefix[1..];
                child.find(slice)
            }
        }
    }

    fn find_or_create(&mut self, prefix: &str) -> &mut PrefixNode<T> {
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

    fn add_all_chars_to_set(&self, set: &mut HashSet<char>) {
        for (key, child) in &self.children {
            set.insert(*key);
            child.add_all_chars_to_set(set);
        }
    }
}

#[derive(Debug)]
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

    pub fn find(&self, prefix: &str) -> Option<&T> {
        self.root.find(prefix).and_then(|node| {
            let value = node.value.as_ref()?;
            Some(value)
        })
    }

    pub fn get_all_chars(&self) -> HashSet<char> {
        let mut set = HashSet::new();
        self.root.add_all_chars_to_set(&mut set);
        set
    }
}

impl<'a, T> FromIterator<(&'a str, T)> for PrefixTree<T> {
    fn from_iter<I: IntoIterator<Item = (&'a str, T)>>(iter: I) -> PrefixTree<T> {
        let mut tree = PrefixTree::new();
        for (key, value) in iter.into_iter() {
            tree.insert(key, value);
        }
        tree
    }
}
