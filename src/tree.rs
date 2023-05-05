use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use camino::Utf8PathBuf;
use indexmap::IndexMap;
use crate::parse::{Command, Entry, Line};

type NodeHandle = Rc<RefCell<Node>>;

#[derive(Default)]
pub struct Node {
    size: usize,
    children: IndexMap<Utf8PathBuf, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl Node {
    pub fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    pub fn total_size(&self) -> u64 {
        self.children
            .values()
            .map(|c| c.borrow().total_size()).
            sum::<u64>()
            + self.size as u64
    }
}

pub fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    // clippy is wrong and should feel bad
    #[allow(clippy::needless_collect)]
    let children = n.borrow().children.values().cloned().collect::<Vec<_>>();

    Box::new(
        std::iter::once(n).chain(
            children
                .into_iter()
                .filter_map(|c| {
                    if c.borrow().is_dir() {
                        Some(all_dirs(c))
                    } else {
                        None
                    }
                })
                .flatten(),
        ),
    )
}

pub struct PrettyNode<'a>(pub &'a NodeHandle);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.size == 0 {
            writeln!(f, "(dir)")?;
        } else {
            writeln!(f, "(file, size={})", this.size)?;
        }

        for (name, child) in &this.children {
            // not very efficient at all, but shrug
            for (index, line) in format!("{:?}", PrettyNode(child)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }
        Ok(())
    }
}

pub fn grow_tree_by_line(mut current_node: NodeHandle, line: Line) -> NodeHandle {
    match line {
        Line::Command(Command::Ls) => {}
        Line::Command(Command::Cd(path)) => {
            match path.as_str() {
                "/" => {}
                ".." => {
                    let parent = current_node.borrow().parent.clone().unwrap();
                    current_node = parent;
                }
                _ => {
                    let child = current_node.borrow_mut().children.entry(path).or_default().clone();
                    current_node = child;
                }
            }
        }
        Line::Entry(Entry::Dir(path)) => {
            let entry = current_node.borrow_mut().children.entry(path).or_default().clone();
            entry.borrow_mut().parent = Some(current_node.clone());
        }
        Line::Entry(Entry::File(size, path)) => {
            let entry = current_node.borrow_mut().children.entry(path).or_default().clone();
            entry.borrow_mut().size = size as usize;
            entry.borrow_mut().parent = Some(current_node.clone());
        }
    }
    current_node
}
