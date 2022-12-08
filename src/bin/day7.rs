use std::fs;
use std::collections::HashMap;
use std::boxed::Box;

// Different type of lines
enum LineTypes {
    Command(Command), // Command, args
    Directory(String), // Directory name
    File(u32, String), // File size, file name
}

enum Command {
    Cd(String),
    Ls,
}

// File system
/*
    File system owns the directories
    Directory owns the files


*/

#[derive(Clone, Debug)]
struct FileSystem {
    directories: HashMap<String, Box<Directory>>,
    directory_sizes: HashMap<String, u32>,
}

#[derive(Clone, Debug)]
struct Directory {
    name: String,
    full_path: String,
    files: Vec<File>,
    child_directories: Vec<String>,
    parent_directory: Option<String>,
}

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u32,
}

impl FileSystem {
    /// Create a new file system from root
    ///
    fn new() -> FileSystem {
        let mut fs = FileSystem {
            directories: HashMap::new(),
            directory_sizes: HashMap::new(),
        };

        fs.directories.insert(String::from("/"), Box::new(Directory::new_root()));

        fs
    }

    fn mkdir(&mut self, dir: Directory) {
        // println!("Creating directory {}", dir.full_path.as_str());
        let parent_dir = self.directories.get_mut(&dir.parent_directory.clone().unwrap()).unwrap();
        parent_dir.child_directories.push(dir.full_path.clone());
        self.directories.insert(dir.full_path.clone(), Box::new(dir));
    }

    fn mkfile(&mut self, dir_full_path: String, file: File) {
        // println!("Creating file {} in directory {}", file.name.as_str(), dir_full_path.as_str());
        let dir = self.directories.get_mut(&dir_full_path).unwrap_or_else(|| panic!("Invalid directory {}", dir_full_path.as_str()));
        dir.add_file(file);
    }

    fn cd(&mut self, dir_full_path: String) -> &Box<Directory> {
        // println!("Changing directory to {}", dir_full_path.as_str());
        self.directories.get(&dir_full_path).unwrap()
    }

    fn size_of(&mut self, dir_full_path: String) -> u32 {
        if self.directory_sizes.contains_key(&dir_full_path) {
            return *self.directory_sizes.get(&dir_full_path).unwrap();
        }

        let mut size = 0;
        // Temporary binding to avoid borrowing self twice
        let binding = self.clone();
        let dir = binding.directories.get(&dir_full_path).unwrap();
        for file in &dir.files {
            size += file.size;
        }
        for child_dir in &dir.child_directories {
            let size_of_child = self.size_of(child_dir.clone());
            size += size_of_child;
        }
        self.directory_sizes.insert(dir_full_path.clone(), size);
        size
    }
}

impl Directory {
    fn new_root() -> Directory {
        Directory {
            name: String::from("/"),
            full_path: String::from("/"),
            files: Vec::new(),
            child_directories: Vec::new(),
            parent_directory: None,
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

}

impl File {
    fn new(name: String, size: u32) -> File {
        File {
            name,
            size,
        }
    }
}


fn parse_line(line: &str) -> LineTypes {
    let first_char = line.chars().next().unwrap();
    let tokens: Vec<&str> = line.split(' ').collect();
    match first_char {
        'd' => LineTypes::Directory(tokens[1].to_string()),
        '0'..='9' => LineTypes::File(tokens[0].parse::<u32>().unwrap(), tokens[1].to_string()),
        '$' => {
            match tokens[1] {
                "cd" => LineTypes::Command(Command::Cd(tokens[2].to_string())),
                "ls" => LineTypes::Command(Command::Ls),
                _ => panic!("Unknown command"),
            }
        },
        _ => panic!("Unknown line type"),
    }
}

fn main() {
    println!("Day 7");
    // Read content
    let content = fs::read_to_string("day 7/input.txt").unwrap();
    let lines = content.lines().collect::<Vec<&str>>();
    // Parse lines
    let mut line_types = Vec::new();
    for line in lines {
        line_types.push(parse_line(line));
    }
    // Create directory structure
    /*
    Own directory structure from the root.
    Traverse through tree by keeping track of location (as a reference) in pwd.
    Generate new directories and files during parsing
    */
    let mut file_system = FileSystem::new();
    let mut pwd = "/".to_string();
    let mut is_ls = false;
    for line_type in line_types {
        match line_type {
            LineTypes::Command(command) => {
                match command {
                    Command::Cd(dir) => {
                        if dir == "/" {
                            pwd = "/".to_string();
                        } else if dir == ".." {
                            pwd = file_system.cd(pwd.clone()).parent_directory.clone().unwrap();
                        } else {
                            pwd += &(dir + "/");
                        }
                    },
                    Command::Ls => {
                        is_ls = true;
                    },
                }
            },
            LineTypes::Directory(dir) => {
                if !is_ls {
                    panic!("Directory without ls");
                }
                let new_dir = Directory {
                    name: dir.clone(),
                    full_path: pwd.clone() + &dir + "/",
                    files: Vec::new(),
                    child_directories: Vec::new(),
                    parent_directory: Some(pwd.clone()),
                };
                file_system.mkdir(new_dir.clone());
            },
            LineTypes::File(size, file) => {
                if !is_ls {
                    panic!("File without ls");
                }
                let new_file = File::new(file.clone(), size);
                file_system.mkfile(pwd.clone(), new_file);
            },
        }
    }

    // Calculate sizes
    let full_size = file_system.size_of("/".to_string());
    let mut size_under_100k = 0;
    for (dir, size) in file_system.directory_sizes.clone() {
        if size <= 100000 {
            size_under_100k += size;
        }
        // println!("{}: {}", dir, size);
    }
    println!("Size: {}", full_size);
    println!("Size under 100k: {}", size_under_100k);

    // Part 2
    let mut smallest_delete = 70000000;
    for (dir, size) in file_system.directory_sizes.clone() {
        if size > (full_size + 30000000 - 70000000) && size < smallest_delete {
            smallest_delete = size;
        }
    }
    println!("Smallest delete: {}", smallest_delete);
}