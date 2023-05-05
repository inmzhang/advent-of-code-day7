use camino::Utf8PathBuf;
use nom::combinator::all_consuming;
use nom::Finish;
use advent_of_code_day7::parse::{Command, Entry, Line, parse_line};

#[derive(Debug)]
struct FsEntry {
    path: Utf8PathBuf,
    size: u64,
    children: Vec<FsEntry>,
}

impl FsEntry {
    fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }

    fn all_dirs(&self) -> Box<dyn Iterator<Item = &FsEntry> + '_> {
        Box::new(
            std::iter::once(self).chain(
                self.children
                    .iter()
                    .filter(|c| !c.children.is_empty())
                    .flat_map(|c| c.all_dirs()),
            ),
        )
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install().unwrap();

    let lines = include_str!("../sample.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let mut stack = vec![FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }];

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
                        let child = stack.pop();
                        stack.last_mut().unwrap().children.push(child.unwrap());

                    }
                    _ => {
                        let node = FsEntry {
                            path: path.clone(),
                            size: 0,
                            children: vec![],
                        };
                        stack.push(node);
                    }
                },
            },
            Line::Entry(entry) => match entry {
                Entry::Dir(_) => {
                    // ignore, we'll do that when we `cd` into them
                }
                Entry::File(size, path) => {
                    let node = FsEntry {
                        size,
                        path,
                        children: vec![],
                    };
                    stack.last_mut().unwrap().children.push(node);
                }
            },
        }
    }

    let root = stack.first_mut().unwrap();
    dbg!(&root);

    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s <= 100_000)
        .sum::<u64>();
    dbg!(sum);

    Ok(())
}
