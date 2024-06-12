//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

mod image;

use crate::image::Image;
use std::io;
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

fn to_file() -> bool {
    let mut text = String::new();
    read_yaml(&mut text);
    let data_array = YamlLoader::load_from_str(&text).unwrap();
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

    let image = Image::new(-6.0, 6.0, -6.0, 6.0, 1200, 1200);
    let mut direction = angle_min;
    /* x-axis: front, y-axis: left */
    for r in ranges {
        match r {
            Real(distance) => {
                if let Ok(d) = distance.parse::<f64>() {
                    let x = f64::cos(direction)*d;
                    let y = f64::sin(direction)*d;
                    eprintln!("{}, {}", x, y);
                }
            },
            _ => {},
        }
        direction += angle_step_width;
    }
    true
}

/*
struct Image {
    pub x_max: f64, //upper edge of image
    pub x_min: f64, //bottom edge of image
    pub y_max: f64, //left edge of image
    pub y_min: f64, //right edge of image
    pub height: usize,
    pub width: usize,
    pub x_step: f64,
    pub y_step: f64,
}

impl Image {
    fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64,
           width: usize, height: usize) -> Image {
        Image {
            x_max: x_max,
            x_min: x_min,
            y_max: y_max,
            y_min: y_min,
            height: height,
            width: width,
            x_step: (x_max - x_min)/(height as f64),
            y_step: (y_max - y_min)/(width as f64),
        }
    }
}

*/
fn main() {
    to_file();
}
