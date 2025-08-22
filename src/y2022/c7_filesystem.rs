use crate::{check_result2, utility};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

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
    Dir(String, Option<Weak<RefCell<Item>>>, Vec<Rc<RefCell<Item>>>),
    File(String, usize),
}

fn parse_entry(parent: &Rc<RefCell<Item>>, first: &str, second: &str) -> Rc<RefCell<Item>> {
    if let Ok(parse) = first.parse() {
        Rc::new(RefCell::new(Item::File(second.to_string(), parse)))
    } else {
        Rc::new(RefCell::new(Item::Dir(
            second.to_string(),
            Some(Rc::downgrade(parent)),
            Vec::new(),
        )))
    }
}

fn parse_filesystem(lines: &str) -> Rc<RefCell<Item>> {
    let root = Rc::new(RefCell::new(Item::Dir("/".to_string(), None, Vec::new())));
    let mut current = root.clone();

    for command in lines.split('$') {
        if command.is_empty() {
            continue;
        }

        let words: Vec<&str> = command.split(' ').collect();

        match words[0] {
            "cd" => match words[1] {
                "/" => current = root.clone(),
                ".." => {
                    current = if let Item::Dir(_, Some(parent_weak), _) = &*current.borrow() {
                        parent_weak.upgrade().expect("Parent should exist")
                    } else {
                        unreachable!("Root or file has no parent")
                    };
                }
                dir_name => {
                    let mut new = current.clone();
                    if let Item::Dir(_, _, children) = &*current.borrow() {
                        for child in children {
                            if let Item::Dir(name, _, _) = &*child.borrow() {
                                if name == dir_name {
                                    new = child.clone();
                                    break;
                                }
                            }
                        }
                    }
                    current = new;
                }
            },
            "ls" => {
                for entry in words[1..].chunks(2) {
                    if let Item::Dir(_, _, children) = &mut *current.borrow_mut() {
                        children.push(parse_entry(&current, entry[0], entry[1]));
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    root
}

fn print_tree(item: &Rc<RefCell<Item>>, indent: usize) {
    let prefix = "  ".repeat(indent);

    match &*item.borrow() {
        Item::File(name, size) => {
            println!("{}- {} (file, size={})", prefix, name, size);
        }
        Item::Dir(name, _, children) => {
            println!("{}- {} (dir)", prefix, name);
            for child in children {
                print_tree(child, indent + 1);
            }
        }
    }
}

fn challenge() -> (usize, usize) {
    let input: String = utility::input::get_input(2022, 7).unwrap();
    let root = parse_filesystem(TEST);

    print_tree(&root, 0);
    (0, 0)
}

check_result2!(0, 0);
