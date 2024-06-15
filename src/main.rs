//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

mod image;

use crate::image::Image;
use std::{env, io};
use yaml_rust::{Yaml, YamlLoader};
use yaml_rust::Yaml::{Array, Integer, Real};

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

fn make_filename(stamp: &Yaml) -> Option<String> {
    let sec = match stamp["sec"] {
        Integer(n) => n,
        _          => return None,
    };
    let nsec = match stamp["nanosec"] {
        Integer(n) => n,
        _          => return None,
    };

    let filename = format!("/tmp/{}.{:09}.pgm", &sec, &nsec);
    Some(filename)
}

fn to_file(x_min: f64, x_max: f64, y_min: f64, y_max: f64,
       width: usize, height: usize, fixed_vote: &mut Image) -> bool {
    let mut text = String::new();
    read_yaml(&mut text);
    let data_array = YamlLoader::load_from_str(&text).unwrap();
    if data_array.len() == 0 {
        return false;
    }

    let data = &data_array[0];

    let filename = match make_filename(&data["header"]["stamp"]) {
        Some(name) => name,
        _          => return false,
    };
    dbg!("{:?}", &filename);

    let angle_min = match &data["angle_min"] {
        Real(x) => x.parse::<f64>().expect("scantopgm: invalid data"),
        _       => return false,
    };
    let angle_step_width = match &data["angle_increment"] {
        Real(x) => x.parse::<f64>().expect("scantopgm: invalid data"),
        _       => return false,
    };

    let ranges = match &data["ranges"] {
        Array(a) => a,
        _       => return false,
    };

    let mut image = Image::new(x_min, x_max, y_min, y_max, width, height);
    let mut direction = angle_min;
    /* x-axis: front, y-axis: left */
    for r in ranges {
        match r {
            Real(distance) => {
                if let Ok(d) = distance.parse::<f64>() {
                    let x = f64::cos(direction)*d;
                    let y = f64::sin(direction)*d;

                    let pix_pos = image.pos_to_pixel(x, y);
                    image.data.insert(pix_pos, 255);

                    match fixed_vote.data.get(&pix_pos) {
                        Some(v) => fixed_vote.data.insert(pix_pos, v+1),
                        None    => fixed_vote.data.insert(pix_pos, 1),
                    };
                }
            },
            _ => {},
        }
        direction += angle_step_width;
    }
    image.pgm_out(&filename);
    true
}

fn main() {
    let args: Vec<String> = env::args().map(|a| a.to_string()).collect();
    let ranges: Vec<f64> = args[1..5].iter().map(|n| n.parse::<f64>().unwrap()).collect();
    let width = args[5].parse::<usize>().unwrap();
    let height = args[6].parse::<usize>().unwrap();

    let mut fixed_vote = Image::new(ranges[0], ranges[1], ranges[2], ranges[3], width, height);

    while to_file(ranges[0], ranges[1], ranges[2], ranges[3],
        width, height, &mut fixed_vote){}

    fixed_vote.normalize();
    fixed_vote.pgm_out("/dev/stdout");
}
