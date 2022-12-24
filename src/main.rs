use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type Tree = Option<Rc<RefCell<TreeNode>>>;

fn new_tree(val: i32) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

#[derive(Debug)]
enum NodeState {
    Left(Tree),
    Right(Tree),
}

impl NodeState {
    fn new(t: &Tree) -> Self {
        NodeState::Left(t.as_ref().cloned())
    }
}

fn mktree(source: &[Option<i32>]) -> Tree {
    let mut source_iter = source.iter();
    let tree: Tree = new_tree(source_iter.next().unwrap().unwrap());
    let mut buff = VecDeque::new();
    buff.push_back(NodeState::new(&tree));
    for val in source_iter {
        let cur = buff.pop_front().unwrap();
        match cur {
            NodeState::Left(Some(node)) => {
                if let Some(val) = val {
                    let subtree = new_tree(*val);
                    buff.push_back(NodeState::new(&subtree.as_ref().cloned()));
                    node.borrow_mut().left = subtree;
                }
                buff.push_front(NodeState::Right(Some(node)));
            }
            NodeState::Right(Some(node)) => {
                if let Some(val) = val {
                    let subtree = new_tree(*val);
                    buff.push_back(NodeState::new(&subtree.as_ref().cloned()));
                    node.borrow_mut().right = subtree;
                }
            }
            _ => {
                panic!();
            }
        }
    }
    tree
}

fn walk(t: Tree) {
    let mut global_depth = 0;
    let mut buf = VecDeque::new();
    buf.push_back((t.as_ref().cloned(), 0));
    loop {
        match buf.pop_front() {
            None => {
                println!("");
                return;
            }
            Some((None, depth)) => {
                if global_depth < depth {
                    println!("");
                    global_depth = depth;
                }
                print!("None ");
            }
            Some((Some(node), depth)) => {
                if global_depth < depth {
                    println!("");
                    global_depth = depth;
                }
                print!("{} ", node.borrow().val);
                buf.push_back((node.borrow().left.as_ref().cloned(), depth + 1));
                buf.push_back((node.borrow().right.as_ref().cloned(), depth + 1));
            }
        }
    }
}

fn main() {
    let tree7 = mktree(&[
        Some(1),
        Some(2), Some(3),
        Some(4), None, None, Some(7),
        Some(8),
    ]);
    walk(tree7);
}
