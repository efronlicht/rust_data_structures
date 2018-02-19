use std::cmp::Ordering;
use std::convert::From;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum BinTree<T: Ord> {
	Node {
		v: T,
		left: Box<BinTree<T>>,
		right: Box<BinTree<T>>,
	},
	Empty,
}




impl<T: Ord> BinTree<T> {
	pub fn new() -> BinTree<T> {BinTree::Empty}

    ///inserts the value into the tree, returning true if the value was successfully inserted,
    ///and false if it was already present
	fn insert(&mut self, new_val: T) -> bool {
		match self {
			&mut BinTree::Node { ref v, ref mut left, ref mut right } => {
				match new_val.cmp(v) {
					Ordering::Equal  => return false, //already present
                    Ordering::Less => right.insert(new_val),
					Ordering::Greater => left.insert(new_val),
					
				};
			},
			&mut BinTree::Empty => {
				*self = BinTree::Node { 
                    v: new_val, 
                    left: Box::new(BinTree::Empty), 
                    right: Box::new(BinTree::Empty) }
			},
		}
        true
	}


    
	pub fn is_empty(&self) -> bool {if let &BinTree::Empty = self {true} else {false}}


	pub fn find(&self, want: T) -> Option<&Self> {
        let mut current = self;
		while let &BinTree::Node{ ref v, ref left, ref right } = current {
            current = match want.cmp(v) {
                    Ordering::Less => right,
                    Ordering::Greater => left,
                    _ => return Some(current)
				};
			};
        None
    }


    pub fn parents_of(&self, val: T) -> Vec<&Self> {
        let mut parents = Vec::new();
        let mut current = self;
		while let &BinTree::Node{ ref v, ref left, ref right } = current {
            let prev = current;
            current = match val.cmp(v) {
                    Ordering::Equal => return parents,
                    Ordering::Less => right,
                    Ordering::Greater => left,
                    
				};
            parents.push(prev);
        };
        panic!("binary tree's invariants are broken!")
    }


    pub fn level(&self, want:T) -> Option<usize> {
        let mut current = self;
        let mut level = 0;
		while let &BinTree::Node{ ref v, ref left, ref right } = current {
            current = match want.cmp(v) {
                    Ordering::Less => right,
                    Ordering::Greater => left,
                    _ => return Some(level),
				};
            level += 1;
            }
        None
    }


}

impl <T: Ord> From<Vec<T>> for BinTree<T> {
    fn from(vec: Vec<T>) -> BinTree<T> {
        let mut tree = BinTree::new();
        for v in vec {
            tree.insert(v);
        }
        tree
    }
}


