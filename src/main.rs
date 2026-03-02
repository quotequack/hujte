use mathlab::functions::*;
use minifb::{Key, Window, WindowOptions};

const GRID: usize = 5;

fn main() {
    let mut window = Window::new("grid", 512, 512, WindowOptions {
        resize: true,
        ..Default::default()
    }).unwrap();

    let mut grid_bits: Vec<Vec<u8>> = vec![vec![]; GRID * GRID];
    let mut cursor = 0usize;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (w, h) = window.get_size();
        let cell_w = w / GRID;
        let cell_h = h / GRID;

        let mut visible: bool = true;
        let mut pressed: Option<u8> = None;
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) { pressed = Some(1); }
        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) { pressed = Some(0); }
        if window.is_key_pressed(Key::LeftBracket, minifb::KeyRepeat::Yes) {
            if cursor != 0 {
                cursor-= 1;
            } else {
                cursor = (GRID*GRID)-1;
            }
            }
        if window.is_key_pressed(Key::RightBracket, minifb::KeyRepeat::Yes) { cursor += 1; if cursor >= GRID * GRID { cursor = 0; } }
        if window.is_key_pressed(Key::Equal, minifb::KeyRepeat::No) { pressed = Some(1); }
        if window.is_key_pressed(Key::Minus, minifb::KeyRepeat::No) { pressed = Some(0); }
        if window.is_key_pressed(Key::Backspace, minifb::KeyRepeat::No) { grid_bits[cursor].pop(); }
        if window.is_key_down(Key::Space) { visible = true; } else { visible = false; }

        if let Some(bit) = pressed {
            grid_bits[cursor].push(bit);
            // cursor += 1;
            if cursor >= GRID * GRID { cursor = 0; }
        }

        let mut buf = vec![0u32; w * h];
        for py in 0..GRID {
            for px in 0..GRID {
                let idx = py * GRID + px;

                let color = if idx == cursor {
                    if visible == false {
                        let bits = &grid_bits[idx];
                        if bits.is_empty() {
                            0xFF6600
                        } else {
                            let mut n: u64 = 0;
                            for &b in bits.iter() {
                                n = (n << 1) | b as u64;
                            }
                            let brightness = calculate_one(n);
                            if brightness >= 0.1 {
                                let v = (brightness * 255.0) as u32;
                                (v << 16) | (v << 4) | v
                            } else {
                                0xFF6600
                            }
                        }
                    } else {
                        let bits = &grid_bits[idx];
                        if bits.is_empty() {
                            0x000000
                        } else {
                            let mut n: u64 = 0;
                            for &b in bits.iter() {
                                n = (n << 1) | b as u64;
                            }
                            let brightness = calculate_one(n);
                            let v = (brightness * 255.0) as u32;
                            (v << 16) | (v << 8) | v
                        }
                    }
                } else {
                    let bits = &grid_bits[idx];
                    if bits.is_empty() {
                        0x000000
                    } else {
                        let mut n: u64 = 0;
                        for &b in bits.iter() {
                            n = (n << 1) | b as u64;
                        }
                        let brightness = calculate_one(n);
                        let v = (brightness * 255.0) as u32;
                        (v << 16) | (v << 8) | v
                    }
                };

                for dy in 0..cell_h {
                    for dx in 0..cell_w {
                        let sx = px * cell_w + dx;
                        let sy = py * cell_h + dy;
                        if sx < w && sy < h {
                            buf[sy * w + sx] = color;
                        }
                    }
                }
            }
        }

        window.update_with_buffer(&buf, w, h).unwrap();
    }
}

fn calculate_one(input: u64) -> f64 {
    if input == 0 { return 0.0; }
    let mut res: f64 = 0.0;
    let length: i32 = (floor(log2(input as f64)) + 1.0) as i32;
    for index in 0..length {
        let quick: i32;
        if index == 0 {
            quick = ((1 << (length - 1)) & (input as i32) != 0) as i32;
        } else {
            quick = ((1 << (length - 1) >> (index)) & (input as i32) != 0) as i32;
        };
        if res > 1.0 { res = 1.0 }
        if index == 0 {
            res = quick as f64
        } else {
            res = abs(((quick - 1 + quick) as f64 * res) + (res / 2.0));
        };
    }
    if res > 1.0 { res = 1.0 }
    res
}