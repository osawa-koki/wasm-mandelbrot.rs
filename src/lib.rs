mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
pub enum HueOption {
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[wasm_bindgen]
pub struct Space {
    width: Option<u32>,
    height: Option<u32>,
    x_min: Option<f64>,
    x_max: Option<f64>,
    y_min: Option<f64>,
    y_max: Option<f64>,
    max_iterations: Option<u32>,
    hue_option: Option<HueOption>,
    cells: Option<Vec<u32>>,
}

#[wasm_bindgen]
impl Space {
    pub fn new() -> Space {
        utils::set_panic_hook();

        Space {
            width: None,
            height: None,
            x_min: None,
            x_max: None,
            y_min: None,
            y_max: None,
            max_iterations: None,
            hue_option: None,
            cells: None,
        }
    }

    pub fn width(&self) -> u32 {
        self.width.unwrap()
    }

    pub fn height(&self) -> u32 {
        self.height.unwrap()
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_ref().unwrap().as_ptr()
    }

    pub fn set_params(
        &mut self,
        width: u32,
        height: u32,
        x_min: f64,
        x_max: f64,
        y_min: f64,
        y_max: f64,
        hue_option: HueOption,
        max_iterations: u32,
    ) {
        self.width = Some(width);
        self.height = Some(height);
        self.x_min = Some(x_min);
        self.x_max = Some(x_max);
        self.y_min = Some(y_min);
        self.y_max = Some(y_max);
        self.hue_option = Some(hue_option);
        self.max_iterations = Some(max_iterations);

        let cells = (0..width * height).map(|_| 255).collect();
        self.cells = Some(cells);
    }

    pub fn compute(&mut self) {
        let width = self.width.unwrap();
        let height = self.height.unwrap();
        let x_min = self.x_min.unwrap();
        let x_max = self.x_max.unwrap();
        let y_min = self.y_min.unwrap();
        let y_max = self.y_max.unwrap();
        let hue_option = self.hue_option.as_mut().unwrap();
        let max_iterations = self.max_iterations.unwrap();

        for y in 0..height {
            for x in 0..width {
                let mut zx = 0.0;
                let mut zy = 0.0;
                let mut zx2 = 0.0;
                let mut zy2 = 0.0;
                let mut i = 0;
                while zx2 + zy2 <= 4.0 && i < max_iterations {
                    zy = 2.0 * zx * zy + (y as f64 / height as f64) * (y_max - y_min) + y_min;
                    zx = zx2 - zy2 + (x as f64 / width as f64) * (x_max - x_min) + x_min;
                    zx2 = zx * zx;
                    zy2 = zy * zy;
                    i += 1;
                }
                // sheet.getRange(y + 1, x + 1).setBackground(rgb2Hex(0, 0, i * 255 / MAX_ITERATIONS))
                let cells = self.cells.as_mut().unwrap();
                let index = (y * width + x) as usize;
                match hue_option {
                    HueOption::Red => {
                        cells[index] = rgb_to_number(i * 255 / max_iterations, 0, 0);
                    }
                    HueOption::Green => {
                        cells[index] = rgb_to_number(0, i * 255 / max_iterations, 0);
                    }
                    HueOption::Blue => {
                        cells[index] = rgb_to_number(0, 0, i * 255 / max_iterations);
                    }
                }
            }
        }
    }
}

fn rgb_to_number(r: u32, g: u32, b: u32) -> u32 {
    (r << 16) + (g << 8) + (b << 0)
}
