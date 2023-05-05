use camino::Utf8PathBuf;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::u64;
use nom::combinator::map;
use nom::IResult;
use nom::sequence::{preceded, separated_pair};

const VALID_CHARS: &str = "abcdefghijklmnopqrstuvwxyz./";

fn parse_path(i: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| VALID_CHARS.contains(c)),
        Into::into,
    )(i)
}

#[derive(Debug)]
struct Ls;

fn parse_ls(i: &str) -> IResult<&str, Ls> {
    map(tag("ls"), |_| Ls)(i)
}

#[derive(Debug)]
struct Cd(Utf8PathBuf);

fn parse_cd(i: &str) -> IResult<&str, Cd> {
    map(preceded(tag("cd "), parse_path), Cd)(i)
}

#[derive(Debug)]
pub enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

impl From<Ls> for Command {
    fn from(_: Ls) -> Self {
        Command::Ls
    }
}

impl From<Cd> for Command {
    fn from(cd: Cd) -> Self {
        Command::Cd(cd.0)
    }
}

fn parse_cmd(i: &str) -> IResult<&str, Command> {
    let (i, _) = tag("$ ")(i)?;
    alt((map(parse_ls, Into::into), map(parse_cd, Into::into)))(i)
}

#[derive(Debug)]
pub enum Entry {
    Dir(Utf8PathBuf),
    File(u64, Utf8PathBuf),
}

fn parse_entry(i: &str) -> IResult<&str, Entry> {
    let parse_file = map(
        separated_pair(u64, tag(" "), parse_path),
        |(size, path)| Entry::File(size, path),
    );
    let parse_dir = map(
        preceded(tag("dir "), parse_path),
        Entry::Dir,
    );
    alt((parse_file, parse_dir))(i)
}

#[derive(Debug)]
pub enum Line {
    Command(Command),
    Entry(Entry),
}

pub fn parse_line(i: &str) -> IResult<&str, Line> {
    alt((
        map(parse_cmd, Line::Command),
        map(parse_entry, Line::Entry),
    ))(i)
}
