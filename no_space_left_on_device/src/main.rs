use std::cell::RefCell;
use std::fs::File;
use std::rc::{Rc, Weak};
use utils::read_lines;

#[allow(dead_code)]
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

        directories
            .iter()
            .map(|d| Rc::clone(d))
            .filter(|directory| directory.name == name)
            .next()
    }

    pub fn size(&self) -> i64 {
        // TODO: can do caching
        let directories = &*self.directories.borrow();
        let files = &*self.files.borrow();

        let total_file_size: i64 = files.iter().map(|file| file.size).sum();
        let total_directory_size: i64 = directories.iter().map(|directory| directory.size()).sum();

        total_file_size + total_directory_size
    }

    pub fn find_directory_of_size_at_most(&self, size: i64) -> i64 {
        let directories = &*self.directories.borrow();
        let directory_sizes = directories
            .iter()
            .map(|directory| directory.find_directory_of_size_at_most(size))
            .sum();
        let directory_size = self.size();
        if directory_size <= size {
            directory_sizes + directory_size
        } else {
            directory_sizes
        }
    }

    pub fn list_of_directories(&self) -> Vec<Rc<DirectoryNode>> {
        let mut current_directories = self.directories.borrow().clone();
        let mut directories: Vec<Rc<DirectoryNode>> = current_directories
            .iter()
            .map(|d| d.list_of_directories())
            .flat_map(|d| d.into_iter())
            .map(|d| Rc::clone(&d))
            .collect();

        directories.append(&mut current_directories);

        directories
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
                        "ls" => {}
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

    pub fn find_directories_of_size_at_most(&self, size: i64) -> i64 {
        self.root.find_directory_of_size_at_most(size)
    }

    pub fn find_smallest_directory_to_free_space(
        &self,
        file_system_size: i64,
        needed_unused_space: i64,
    ) -> i64 {
        let current_used_space = self.root.size();
        let current_free_space = file_system_size - current_used_space;
        let space_to_free_up = needed_unused_space - current_free_space;

        let directories = self.root.list_of_directories();

        let mut sizes: Vec<i64> = directories.iter().map(|d| d.size()).collect();

        sizes.sort();

        sizes
            .into_iter()
            .filter(|s| *s >= space_to_free_up)
            .next()
            .unwrap()
    }
}

fn main() {
    let file = File::open("no_space_left_on_device/input/input.txt").unwrap();
    let mut lines = read_lines(file);
    let system = System::new(&mut lines);

    println!(
        "Part 1: total size of directory with size at most 100000 - {:?}",
        system.find_directories_of_size_at_most(100000)
    );
    println!(
        "Part 2: size of directory to delete to free up space - {:?}",
        system.find_smallest_directory_to_free_space(70000000, 30000000)
    );
}
