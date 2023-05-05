use std::cell::RefCell;
use std::rc::Rc;
use nom::combinator::all_consuming;
use nom::Finish;
use advent_of_code_day7::parse::parse_line;
use advent_of_code_day7::tree::{all_dirs, grow_tree_by_line, Node};

fn main() {
    let lines = include_str!("../sample.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);
    let root = Rc::new(RefCell::new(Node::default()));
    let mut node = root.clone();
    for line in lines {
        // println!("{line:?}");
        node = grow_tree_by_line(node, line);
    }
    // println!("{:#?}", PrettyNode(&root));
    let total_space = 70000000_u64;
    let used_space = root.borrow().total_size();
    let free_space = total_space.checked_sub(dbg!(used_space)).unwrap();
    let needed_free_space = 30000000_u64;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();

    let removed_dir_size = all_dirs(root)
        .map(|d| d.borrow().total_size())
        .filter(|&s| s >= minimum_space_to_free)
        .inspect(|s| {
            dbg!(s);
        })
        .min();
    dbg!(removed_dir_size);
}
