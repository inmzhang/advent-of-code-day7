use camino::Utf8PathBuf;
use id_tree::{InsertBehavior, Node, Tree};
use nom::combinator::all_consuming;
use nom::Finish;
use advent_of_code_day7::parse::{Command, Entry, Line, parse_line};

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
}

fn total_size(tree: &Tree<FsEntry>, node: &Node<FsEntry>) -> color_eyre::Result<u64> {
    let mut total = node.data().size;
    for child in node.children() {
        total += total_size(tree, tree.get(child)?)?;
    }
    Ok(total)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let lines = include_str!("../sample.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut tree = Tree::<FsEntry>::new();
    let root = tree.insert(
        Node::new(FsEntry {
            path: "/".into(),
            size: 0,
        }),
        InsertBehavior::AsRoot,
    )?;
    let mut curr = root;

    for line in lines {
        println!("{line:?}");
        match line {
            Line::Command(cmd) => match cmd {
                Command::Ls => {
                    // just ignore those
                }
                Command::Cd(path) => match path.as_str() {
                    "/" => {
                        // ignore, we're already there
                    }
                    ".." => {
                        curr = tree.get(&curr)?.parent().unwrap().clone();
                    }
                    _ => {
                        let node = Node::new(FsEntry {
                            path: path.clone(),
                            size: 0,
                        });
                        curr = tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, path) => {
                    let node = Node::new(FsEntry { size, path });
                    tree.insert(node, InsertBehavior::UnderNode(&curr))?;
                }
            },
        }
    }

    let mut s = String::new();
    tree.write_formatted(&mut s)?;
    println!("{s}");

    let sum = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        // only consider folders:
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s <= 100_000)
        .inspect(|s| {
            dbg!(s);
        })
        .sum::<u64>();
    dbg!(sum);

    let total_space = 70000000_u64;
    let used_space = total_size(&tree, tree.get(tree.root_node_id().unwrap())?)?;
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let size_to_remove = tree
        .traverse_pre_order(tree.root_node_id().unwrap())?
        .filter(|n| !n.children().is_empty())
        .map(|n| total_size(&tree, n).unwrap())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();
    dbg!(size_to_remove);

    Ok(())
}
