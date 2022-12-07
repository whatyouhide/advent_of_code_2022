use core::fmt;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

const AVAILABLE_SPACE: u64 = 70000000;
const MIN_FREE_SPACE: u64 = 30000000;

#[derive(Debug)]
pub struct File {
    pub name: String,
    size: u64,
}

#[derive(Debug)]
pub struct Dir {
    pub name: String,
}

#[derive(Debug)]
enum NodeValue {
    File(File),
    Dir(Dir),
}

struct Node {
    value: NodeValue,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("value", &self.value)
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    pub fn new(dir: Dir) -> Node {
        Node {
            value: NodeValue::Dir(dir),
            children: HashMap::new(),
            parent: None,
        }
    }

    fn append_node<'a>(&mut self, name: String, node: Rc<RefCell<Node>>) {
        self.children.insert(name.clone(), node);
    }

    fn size(&self) -> u64 {
        match &self.value {
            NodeValue::File(file) => file.size,
            NodeValue::Dir(_) => {
                let mut size = 0;
                for child in self.children.values() {
                    size += child.borrow().size();
                }
                size
            }
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug)]
enum Line {
    Command(Command),
    FileWithSize(String, u64),
    Dir(String),
}

pub fn run() {
    let input = include_str!("../inputs/day7.txt");

    let root = Rc::new(RefCell::new(Node::new(Dir {
        name: "/".to_string(),
    })));

    let mut current_dir = Rc::clone(&root);

    let lines = input.lines().skip(1).map(parse_line);

    for line in lines {
        match line {
            // For a file, we create a new file struct and add it to the current directory's
            // children.
            Line::FileWithSize(filename, size) => {
                let child_node = Rc::new(RefCell::new(Node {
                    value: NodeValue::File(File {
                        name: filename.to_string(),
                        size,
                    }),
                    children: HashMap::new(),
                    parent: None,
                }));

                current_dir
                    .borrow_mut()
                    .append_node(filename, Rc::clone(&child_node));

                child_node.borrow_mut().parent = Some(Rc::clone(&current_dir));
            }

            // For a directory, we create a new directory struct, set its parent
            // the current directory, and add it to the current directory's
            // children.
            Line::Dir(dir_name) => {
                let child_node = Rc::new(RefCell::new(Node::new(Dir {
                    name: dir_name.to_string(),
                })));

                current_dir
                    .borrow_mut()
                    .append_node(dir_name, Rc::clone(&child_node));

                let mut mut_child = child_node.borrow_mut();
                mut_child.parent = Some(Rc::clone(&current_dir));
            }

            // "ls" is kind of not very useful, so we just ignore it.
            Line::Command(Command::Ls) => continue,

            // For "cd", we find the directory in the current directory's children
            // and make it the current directory. If the directory is "/", we go
            // back to the root. If the directory is "..", we go to the parent
            // of the current directory.
            Line::Command(Command::Cd(dir_name)) => match dir_name.as_str() {
                "/" => {
                    current_dir = Rc::clone(&root);
                }
                ".." => {
                    let current_dir_clone = Rc::clone(&current_dir);
                    current_dir = Rc::clone(current_dir_clone.borrow().parent.as_ref().unwrap());
                }
                _ => {
                    let child_clone = Rc::clone(&current_dir.borrow().children[&dir_name]);
                    current_dir = child_clone;
                }
            },
        }
    }

    let small_size = calc_size(&root.borrow());
    println!("Total size of dirs < 100k: {:?}", small_size);

    let total_size = root.borrow().size();

    println!(
        "Total size of all dirs: {:?} (which leaves {} free)",
        total_size,
        AVAILABLE_SPACE - total_size
    );

    println!(
        "Size of the smallest dir to delete: {:?}",
        find_size_of_smallest_dir_to_delete(total_size, &root.borrow())
    );
}

fn find_size_of_smallest_dir_to_delete(total_size: u64, node: &Node) -> u64 {
    let mut smallest_dir_size = u64::MAX;
    let size = node.size();

    if viable_size(total_size, size) && size < smallest_dir_size {
        smallest_dir_size = size;
    }

    for child in node.children.values() {
        match child.borrow().value {
            NodeValue::File(_) => continue,
            NodeValue::Dir(_) => {
                let child_size = find_size_of_smallest_dir_to_delete(total_size, &child.borrow());

                if viable_size(total_size, size) && child_size < smallest_dir_size {
                    smallest_dir_size = child_size;
                }
            }
        }
    }

    return smallest_dir_size;
}

fn viable_size(total_size: u64, size: u64) -> bool {
    (AVAILABLE_SPACE - total_size + size) >= MIN_FREE_SPACE
}

fn calc_size(node: &Node) -> u64 {
    let mut total = 0;
    let size = node.size();

    if size <= 100000 {
        total += size;
    }

    for child in node.children.values() {
        match child.borrow().value {
            NodeValue::File(_) => continue,
            NodeValue::Dir(_) => {
                total += calc_size(&child.borrow());
            }
        }
    }

    return total;
}

fn parse_line(line: &str) -> Line {
    if line.starts_with("$") {
        let command = line[2..].trim();

        if command.starts_with("cd") {
            let dir = command[2..].trim();
            Line::Command(Command::Cd(dir.to_string()))
        } else if command == "ls" {
            Line::Command(Command::Ls)
        } else {
            panic!("Unknown command: {}", command);
        }
    } else {
        if line.starts_with("dir") {
            let dir_name = line[4..].trim();
            Line::Dir(dir_name.to_string())
        } else {
            let mut parts = line.split_whitespace();
            let size: u64 = parts.next().unwrap().to_string().parse::<u64>().unwrap();
            let name = parts.next().unwrap().to_string();
            Line::FileWithSize(name, size)
        }
    }
}
