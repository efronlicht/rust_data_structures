
use super::*;
/// for our tests, 
/// we want a tree like this:
///        2
///    -1    5
/// <> <>  4   8
///      <> <> <> <>
#[test]
fn level() {
    /// we want a tree like this:
    ///     2
    ///  -1   5
    ///      4  8
    let mut tree = BinTree::new();
    tree.insert(2); 
    tree.insert(-1); tree.insert(5);
    tree.insert(4); tree.insert(8);
    assert_eq!(tree.level(2), Some(0));
    assert_eq!(tree.level(4), tree.level(8));
    assert_eq!(tree.level(-1), tree.level(5));
}

#[test]
fn from_vec() {
    let tree = BinTree::from(vec!(2, -1, 5, 4, 8));
    assert_eq!(tree.level(2), Some(0));
    assert_eq!(tree.level(4), tree.level(8));
    assert_eq!(tree.level(-1), tree.level(5));
}

#[test]
fn parents_of() {
    let tree = BinTree::from(vec!(2, -1, 5, 4, 8));
    let node_8_parents = tree.parents_of(8);
    assert_eq!(node_8_parents.len(), 2);
    if let &BinTree::Node{ref v, ..} = node_8_parents[0] {
        assert_eq!(v, &2);
    } else {
        panic!("should have a node")
    }

    if let &BinTree::Node{ref v, ..} = node_8_parents[1] {
        assert_eq!(v, &5);
    } else {
        panic!("should have a node")
    }
    
}

