use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::fs::File;
use std::rc::{Rc, Weak};
use utils::read_lines;

#[derive(Debug)]
struct FileNode {
    name: String,
    size: i64,
    parent: Weak<DirectoryNode>,
}

impl FileNode {
    pub fn new(name: String, size: i64, parent: Weak<DirectoryNode>) -> Rc<Self> {
        Rc::new(FileNode { name, size, parent })
    }
}

#[derive(Debug)]
struct DirectoryNode {
    name: String,
    directories: RefCell<Vec<Rc<DirectoryNode>>>,
    files: RefCell<Vec<Rc<FileNode>>>,
    parent: Weak<DirectoryNode>,
}

impl DirectoryNode {
    pub fn new(name: String, parent: Weak<DirectoryNode>) -> Rc<Self> {
        Rc::new(DirectoryNode {
            name,
            directories: RefCell::new(vec![]),
            files: RefCell::new(vec![]),
            parent,
        })
    }

    pub fn add_file(&self, file: Rc<FileNode>) {
        self.files.borrow_mut().push(file);
    }

    pub fn add_directory(&self, directory: Rc<DirectoryNode>) {
        self.directories.borrow_mut().push(directory);
    }

    pub fn find_directory(&self, name: &str) -> Option<Rc<DirectoryNode>> {
        let directories = &*self.directories.borrow();

        directories.iter()
            .map(|d| Rc::clone(d))
            .filter(|directory| directory.name == name)
            .next()
    }
}

#[derive(Debug)]
struct System {
    root: Rc<DirectoryNode>,
}

impl System {
    pub fn new(commands: &mut Vec<String>) -> Self {
        let root = DirectoryNode::new("/".to_string(), Weak::new());

        let mut current = Rc::clone(&root);

        commands
            .into_iter()
            .map(|s| s.split_whitespace().into_iter())
            .for_each(|mut command| {
                match command.next().unwrap() {
                    "$" => match command.next().unwrap() {
                        "cd" => match command.next().unwrap() {
                            ".." => {
                                current = current.parent.upgrade().unwrap();
                            }
                            "/" => {
                                current = Rc::clone(&root);
                            }
                            directory => {
                                current = current.find_directory(directory).unwrap();
                            }
                        },
                        "ls" => {},
                        _ => {
                            panic!()
                        }
                    },
                    "dir" => {
                        // TODO: check if directory is already registered
                        let directory = DirectoryNode::new(
                            command.next().unwrap().to_string(),
                            Rc::downgrade(&current),
                        );
                        current.add_directory(directory);
                    }
                    file_size => {
                        // TODO: check if file is already registered
                        let file = FileNode::new(
                            command.next().unwrap().to_string(),
                            file_size.parse().unwrap(),
                            Rc::downgrade(&current),
                        );
                        current.add_file(file);
                    }
                }
            });

        System { root }
    }
}

fn main() {
    let file = File::open("no_space_left_on_device/input/input.txt").unwrap();
    let mut lines = read_lines(file);
    let system = System::new(&mut lines);

    println!("{:#?}", system.root);
}
