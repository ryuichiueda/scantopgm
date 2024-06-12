//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::io;
use yaml_rust::YamlLoader;
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

fn to_file() -> bool {
    let mut text = String::new();
    read_yaml(&mut text);
    let data = YamlLoader::load_from_str(&text).unwrap();
    let header = &data[0]["header"];

    let sec = match header["stamp"]["sec"] {
        Integer(n) => n,
        _          => return false,
    };

    let nsec = match header["stamp"]["nanosec"] {
        Integer(n) => n,
        _          => return false,
    };

    let filename = format!("{}.{:09}.pgm", &sec, &nsec);
    dbg!("{:?}", &filename);
    true
}

fn main() {
    to_file();
}
