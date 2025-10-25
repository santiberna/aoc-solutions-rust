use crate::utility::tree::Node;
use crate::{check_result2, utility};
use std::cell::RefCell;
use std::ops::Deref;
use std::process::Output;
use std::rc::{Rc, Weak};
use std::str::FromStr;

const TEST: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

#[derive(Debug)]
enum Item {
    Directory(String),
    File(String, usize),
}

impl Item
{
    fn name(&self) -> &str {
        match self {
            Item::Directory(name) => name,
            Item::File(name, _) => name
        }
    }
}

#[derive(Debug)]
enum Command {
    Ls(Vec<Item>),
    Cd(String),
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = if s.starts_with("ls") {
            Command::Ls(s.lines().skip(1).map(|e| e.parse().unwrap()).collect())
        } else if s.starts_with("cd") {
            Command::Cd(s.trim().split(' ').skip(1).next().unwrap().to_string())
        } else {
            panic!("Found unknown command {}", &s);
        };
        Ok(command)
    }
}

impl FromStr for Item {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words = s.split(' ').collect::<Vec<_>>();
        Ok(match words[0] {
            "dir" => Item::Directory(words[1].to_string()),
            _ => Item::File(words[1].to_string(), words[0].parse().unwrap()),
        })
    }
}

fn parse_filesystem(lines: &str) -> Rc<Node<Item>> {

    let commands = lines.split("$ ").skip(1).map(|e| e.parse().unwrap()).collect::<Vec<Command>>();
    let root = Node::new(Item::Directory("/".to_string()));
    let mut current = root.clone();

    for command in commands {
        match command {
            Command::Cd(target) => {
                match target.as_str() {
                    ".." => current = current.parent().unwrap(),
                    "/" => current = root.clone(),
                    _ => {
                        let found = current.children().iter().find(|&e| target == e.name()).cloned();

                        if let Some(found) = found {
                            current = found.clone()
                        } else {
                            let new = Node::new(Item::Directory(target.clone()));
                            current = new.clone();
                            Node::add_child(&current, new);
                        }
                    }
                }
            }
            Command::Ls(contents) => {
;                for item in contents {
                    let found = current.children().iter().find(|&e| item.name() == e.name()).cloned();

                    if found.is_none() {
                        Node::add_child(&current, Node::new(item));
                    }
                }
            }
        }
    }

    root
}

fn visit<F>(item: &Rc<Node<Item>>, visitor: &mut F) where F: FnMut(&Node<Item>)  {
    visitor(item.deref());
    for child in item.children().iter() {
        visit(child, visitor);
    }
}

fn directory_size(node: &Node<Item>) -> usize {
    let mut accum = 0;
    for child in node.children().iter() {
        match child.deref().deref() {
            Item::Directory(_) => accum += directory_size(child),
            Item::File(_, size) => accum += size
        }
    }
    accum
}


fn challenge() -> (usize, usize) {
    let input: String = utility::input::get_input(2022, 7).unwrap();
    let root = parse_filesystem(&input);

    let mut directories: Vec<(String, usize)> = vec![];

    let mut visitor = |node: &Node<Item>| {
        match node.deref() {
            Item::Directory(name) => directories.push((name.clone(), directory_size(node))),
            _ => {}
        }
    };

    visit(&root, &mut visitor);
    let sum = directories.iter().skip(1).filter(|(_, size)| *size <= 100000).fold(0, |a, (_, s)| a + s);
    let total_occupied = directories.first().unwrap().1;
    let total_free = 70000000 - total_occupied;
    let total_necessary = 30000000 - total_free;

    let min_necessary = directories.iter().map(|(_, s)| s).filter(|s| **s >= total_necessary).min().unwrap();

    (sum, *min_necessary)
}

check_result2!(1844187, 4978279);
