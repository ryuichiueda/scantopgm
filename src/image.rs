//SPDX-FileCopyrightText: 2024 Ryuichi Ueda ryuichiueda@gmail.com
//SPDX-License-Identifier: BSD-3-Clause

pub struct Image {
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
    pub fn new(x_min: f64, x_max: f64, y_min: f64, y_max: f64,
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
