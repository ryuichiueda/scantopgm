//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

use std::io;

fn main() {
    loop {
        let mut line = String::new();
        let len = io::stdin()
                .read_line(&mut line)
                .expect("scantopgm: Failed to read line");
    
        if len == 0 {
            break;
        }
        eprintln!("{:?}", &line);
    }
}
