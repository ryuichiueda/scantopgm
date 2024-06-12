//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::io;
use yaml_rust::YamlLoader;
use yaml_rust::Yaml;
use yaml_rust::Yaml::Integer;

fn read_yaml(text: &mut String) {
    loop {
        let mut line = String::new();

        if io::stdin().read_line(&mut line)
           .expect("scantopgm: Failed to read line") == 0 
        || line.starts_with("---") {
            break;
        }

        *text += &line;
    }
}

fn make_filename(header: &Yaml) -> Option<String> {
    let sec = match header["stamp"]["sec"] {
        Integer(n) => n,
        _          => return None,
    };
    let nsec = match header["stamp"]["nanosec"] {
        Integer(n) => n,
        _          => return None,
    };

    let filename = format!("/tmp/{}.{:09}.pgm", &sec, &nsec);
    Some(filename)
}

fn to_file() -> bool {
    let mut text = String::new();
    read_yaml(&mut text);
    let data = YamlLoader::load_from_str(&text).unwrap();
    let header = &data[0]["header"];

    let filename = make_filename(&header);
    dbg!("{:?}", &filename);
    true
}

fn main() {
    to_file();
}
