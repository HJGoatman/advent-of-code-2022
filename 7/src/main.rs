use std::fs;

type FileSize = u32;
type FileName = String;
type DirectoryName = String;

#[derive(Debug, PartialEq, Clone)]
enum FileSystem {
    File(FileName, FileSize),
    Directory(DirectoryName, Vec<FileSystem>),
}

#[derive(Debug, PartialEq)]
enum ListOutput {
    Dir(DirectoryName),
    File(FileName, FileSize),
}

#[derive(Debug, PartialEq)]
enum Command {
    ChangeDirectory(String),
    List(Vec<ListOutput>),
}

fn load_input() -> String {
    fs::read_to_string("input.txt").expect("Should have been able to read the file")
}

fn parse_cd(input: &str) -> Command {
    Command::ChangeDirectory(input[3..input.len() - 1].to_string())
}

fn parse_ls_out_dir(input: &str) -> ListOutput {
    ListOutput::Dir(input[4..].to_string())
}

fn parse_ls_out_file(input: &str) -> ListOutput {
    let file_string_split: Vec<&str> = input.split(" ").collect();
    let size = file_string_split
        .get(0)
        .unwrap()
        .parse::<FileSize>()
        .unwrap();
    let name = file_string_split.get(1).unwrap().to_string();
    ListOutput::File(name, size)
}

fn parse_list_output(input: &str) -> ListOutput {
    match &input[..3] {
        "dir" => parse_ls_out_dir(&input),
        _ => parse_ls_out_file(&input),
    }
}

fn parse_ls(input: &str) -> Command {
    Command::List(
        input
            .split('\n')
            .filter(|a| a != &"")
            .skip(1)
            .map(parse_list_output)
            .collect(),
    )
}

fn parse_command(input: &str) -> Command {
    match &input[0..2] {
        "cd" => parse_cd(&input),
        "ls" => parse_ls(&input),
        &_ => todo!(),
    }
}

fn parse_input(input: &str) -> Vec<Command> {
    input.split("$ ").skip(1).map(parse_command).collect()
}

fn set_at_path<'a>(filesystem: &'a mut FileSystem, path: &[String], set_value: Vec<FileSystem>) {
    if let FileSystem::Directory(name, children) = filesystem {
        let head = &path[0];

        if head == name {
            if path.len() == 1 {
                *children = set_value;
                return;
            }

            for child in children {
                set_at_path(child, &path[1..], set_value.clone());
            }
        }
    }
}

fn list_to_filesystem(list: &[ListOutput]) -> Vec<FileSystem> {
    list.iter()
        .map(|item| match item {
            ListOutput::Dir(dir) => FileSystem::Directory(dir.clone(), Vec::new()),
            ListOutput::File(name, size) => FileSystem::File(name.to_string(), size.clone()),
            _ => unreachable!(),
        })
        .collect()
}

fn add_children<'a>(
    filesystem: &'a mut FileSystem,
    path: &[String],
    list_output: &[ListOutput],
) -> &'a mut FileSystem {
    let subfilesystem = list_to_filesystem(list_output);

    set_at_path(filesystem, path, subfilesystem);

    filesystem
}

fn build_filesystem(commands: &[Command]) -> FileSystem {
    let mut root = FileSystem::Directory(String::from("/"), Vec::new());
    let mut path = Vec::new();

    for command in commands {
        if let Command::ChangeDirectory(dir) = command {
            if dir == ".." {
                path.pop();
                continue;
            }

            path.push(dir.clone());
        }

        if let Command::List(list_output) = command {
            add_children(&mut root, &path, &list_output);
        }
    }

    root
}

fn get_all_directories(filesystem: &FileSystem) -> Vec<FileSystem> {
    if let FileSystem::Directory(name, children) = filesystem {
        let mut dirs = vec![filesystem.clone()];
        let mut child_dirs: Vec<FileSystem> =
            children.iter().flat_map(&get_all_directories).collect();
        dirs.append(&mut child_dirs);
        return dirs;
    } else {
        vec![]
    }
}

fn get_directory_size(filesystem: &FileSystem) -> FileSize {
    match filesystem {
        FileSystem::File(_, size) => *size,
        FileSystem::Directory(_, children) => children.iter().map(get_directory_size).sum(),
    }
}

fn sum_directory_size(root: &FileSystem) -> FileSize {
    let directories = get_all_directories(&root);
    directories
        .iter()
        .map(get_directory_size)
        .filter(|size| size <= &100000)
        .sum()
}

fn main() {
    let input = load_input();
    let commands = parse_input(&input);
    let filesystem = build_filesystem(&commands);
    let directory_sizes = sum_directory_size(&filesystem);
    println!("{}", directory_sizes);
}

#[cfg(test)]
mod tests {
    use crate::{
        build_filesystem, parse_input, sum_directory_size, Command, FileSystem, ListOutput,
    };

    fn get_test_commands() -> Vec<Command> {
        vec![
            Command::ChangeDirectory(String::from("/")),
            Command::List(vec![
                ListOutput::Dir(String::from("a")),
                ListOutput::File(String::from("b.txt"), 14848514),
                ListOutput::File(String::from("c.dat"), 8504156),
                ListOutput::Dir(String::from("d")),
            ]),
            Command::ChangeDirectory(String::from("a")),
            Command::List(vec![
                ListOutput::Dir(String::from("e")),
                ListOutput::File(String::from("f"), 29116),
                ListOutput::File(String::from("g"), 2557),
                ListOutput::File(String::from("h.lst"), 62596),
            ]),
            Command::ChangeDirectory(String::from("e")),
            Command::List(vec![ListOutput::File(String::from("i"), 584)]),
            Command::ChangeDirectory(String::from("..")),
            Command::ChangeDirectory(String::from("..")),
            Command::ChangeDirectory(String::from("d")),
            Command::List(vec![
                ListOutput::File(String::from("j"), 4060174),
                ListOutput::File(String::from("d.log"), 8033020),
                ListOutput::File(String::from("d.ext"), 5626152),
                ListOutput::File(String::from("k"), 7214296),
            ]),
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = "$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k";
        let expected = get_test_commands();
        let actual = parse_input(&input);
        assert_eq!(expected, actual);
    }

    fn get_test_filesystem() -> FileSystem {
        FileSystem::Directory(
            String::from("/"),
            vec![
                FileSystem::Directory(
                    String::from("a"),
                    vec![
                        FileSystem::Directory(
                            String::from("e"),
                            vec![FileSystem::File(String::from("i"), 584)],
                        ),
                        FileSystem::File(String::from("f"), 29116),
                        FileSystem::File(String::from("g"), 2557),
                        FileSystem::File(String::from("h.lst"), 62596),
                    ],
                ),
                FileSystem::File(String::from("b.txt"), 14848514),
                FileSystem::File(String::from("c.dat"), 8504156),
                FileSystem::Directory(
                    String::from("d"),
                    vec![
                        FileSystem::File(String::from("j"), 4060174),
                        FileSystem::File(String::from("d.log"), 8033020),
                        FileSystem::File(String::from("d.ext"), 5626152),
                        FileSystem::File(String::from("k"), 7214296),
                    ],
                ),
            ],
        )
    }

    #[test]
    fn test_build_filesystem() {
        let input = get_test_commands();
        let expected = get_test_filesystem();
        let actual = build_filesystem(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_sum_directory_size() {
        let input = get_test_filesystem();
        let expected = 95437;
        let actual = sum_directory_size(&input);
        assert_eq!(expected, actual);
    }
}
