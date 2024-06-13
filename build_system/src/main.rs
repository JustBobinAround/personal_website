use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct File {
    name: String,
    size: u64,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Directory {
    name: String,
    files: Vec<File>,
    subdirectories: Vec<Directory>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FileSystem {
    root: Directory,
}

#[derive(Serialize, Deserialize, Debug)]
enum Color {
    Normal,
    LightGrey,
    DarkBlue,
    LightBlue,
    Red,
}

#[derive(Serialize, Deserialize, Debug)]
struct CharProps {
    is_bold: bool,
    is_ital: bool,
    is_curs: bool,
    is_hili: bool,
    is_link: bool,
    char: char,
    color: Color,
    link: String
}

#[derive(Serialize, Deserialize, Debug)]
struct CharLine {
    chars: Vec<CharProps>
}

#[derive(Serialize, Deserialize, Debug)]
struct FileContent {
    lines: Vec<CharLine>
}

fn main() {
    let file1 = File {
        name: String::from("file1.txt"),
        size: 100,
        url: String::from("./files/hash1") 
    };

    let file2 = File {
        name: String::from("file2.txt"),
        size: 200,
        url: String::from("./files/hash2") 
    };

    let subdirectory = Directory {
        name: String::from("subdir"),
        files: vec![file2],
        subdirectories: vec![],
    };

    let root_directory = Directory {
        name: String::from("root"),
        files: vec![file1],
        subdirectories: vec![subdirectory],
    };

    let file_system = FileSystem {
        root: root_directory,
    };

    let char_prop_l = CharProps {
        is_bold: true,
        is_ital: false,
        is_curs: false,
        is_hili: false,
        is_link: true,
        char: 'l',
        color:Color::Red,
        link:  String::from("./files/hash1")
    };
    let char_prop_m = CharProps {
        is_bold: true,
        is_ital: false,
        is_curs: false,
        is_hili: false,
        is_link: true,
        char: 'm',
        color:Color::Red,
        link:  String::from("./files/hash1")
    };
    let char_prop_a = CharProps {
        is_bold: true,
        is_ital: false,
        is_curs: false,
        is_hili: false,
        is_link: true,
        char: 'a',
        color: Color::Red,
        link:  String::from("./files/hash1")
    };
    let char_prop_o = CharProps {
        is_bold: true,
        is_ital: false,
        is_curs: false,
        is_hili: false,
        is_link: true,
        char: 'o',
        color:Color::Red,
        link:  String::from("./files/hash1")
    };

    let char_line = CharLine {
        chars: vec![
            char_prop_l,
            char_prop_m,
            char_prop_a,
            char_prop_o,
        ]
    };

    let content = FileContent {
        lines: vec![char_line]
    };

    let serialized = serde_json::to_string(&file_system).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: FileSystem = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);

    let serialized = serde_json::to_string(&content).unwrap();
    println!("Serialized: {}", serialized);

    let deserialized: FileContent = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized: {:?}", deserialized);
}

