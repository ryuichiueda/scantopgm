//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::io;

fn header_to_filename() -> Option<String> {
    let mut sec = String::new();

    loop {
        let mut line = String::new();

        if io::stdin().read_line(&mut line)
           .expect("scantopgm: Failed to read line") == 0 {
            break;
        }

        let line = line.trim().to_string();
        let name = line.split(":").nth(0).unwrap_or("");
        let data = line.split(":").nth(1).unwrap_or("");

        match name {
            "header" | "stamp" => {}, 
            "sec" => sec = data.to_string(),
            "nanosec" => {
                let ans = format!("{}.{:>09}", sec, data.trim()).replace(" ", "0");
                return Some(ans);
            },
            _ => {},
        }
    }

    None
}

fn main() {
    loop {
        let filename = header_to_filename();
        if filename == None {
            return;
        }
        println!("{:?}", filename);
    }
}
