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

impl FsEntry {
    fn build(mut self, it: &mut dyn Iterator<Item = Line>) -> Self {
        while let Some(line) = it.next() {
            match line {
                Line::Command(Command::Cd(sub)) => match sub.as_str() {
                    "/" => {
                        // muffin,
                    }
                    ".." => break,
                    _ => self.children.push(
                        FsEntry {
                            path: sub.clone(),
                            size: 0,
                            children: vec![],
                        }
                            .build(it),
                    ),
                },
                Line::Entry(Entry::File(size, path)) => {
                    self.children.push(FsEntry {
                        path,
                        size,
                        children: vec![],
                    });
                }
                _ => {
                    // ignore other commands
                }
            }
        }
        self
    }
}

fn main() {
    let mut lines = include_str!("../sample.txt")
        .lines()
        .map(|l| all_consuming(parse_line)(l).finish().unwrap().1);

    let root = FsEntry {
        path: "/".into(),
        size: 0,
        children: vec![],
    }
        .build(&mut lines);
    dbg!(&root);

    // solving part 1 because it's the same difficulty as part 2, just less code
    let sum = root
        .all_dirs()
        .map(|d| d.total_size())
        .filter(|&s| s < 100_000)
        .sum::<u64>();
    dbg!(sum);
}