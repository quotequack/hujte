use mathlab::functions::*;
use minifb::{Key, Window, WindowOptions};

const GRID: usize = 32;

fn main() {
    let mut window = Window::new("grid", 512, 512, WindowOptions {
        resize: true,
        ..Default::default()
    }).unwrap();

    let mut grid = [0u8; GRID * GRID]; // 0 = black, 1 = white
    let mut cursor = 0; // current pixel index

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (w, h) = window.get_size();
        let cell_w = w / GRID;
        let cell_h = h / GRID;

        // handle input
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            if cursor < GRID * GRID { grid[cursor] = 1; cursor += 1; }
        }
        if window.is_key_pressed(Key::Key0, minifb::KeyRepeat::No) {
            if cursor < GRID * GRID { grid[cursor] = 0; cursor += 1; }
        }

        // draw
        let mut buf = vec![0u32; w * h];
        for py in 0..GRID {
            for px in 0..GRID {
                let color = if grid[py * GRID + px] == 1 { 0xFFFFFF } else { 0x000000 };
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
    };
    let input:u64 = 1;
    println!("i:{:b}", &input);
    let res = calculate_one(input);
    println!("{}",res)
}
fn calculate_one(input:u64) -> f64 {
    let mut res:f64 = 0.0;
    let length: i32 = (floor(log2(input as f64))+1.0) as i32;
    for index in 0..length {
        // println!("starting index {} with res {}", &index, &res);
        let quick:i32;
        if index == 0 {
            quick = ((1<<(length-1))&(input as i32) != 0) as i32;
            // println!("mask0: {:b}",(1<<(length-1)));
            // println!("dig 0: {:b}",&quick);
        } else {
            // println!("mask{}: {:b}",&index ,(1<<(length-1)>>(index)));
            quick = ((1<<(length-1)>>(index))&(input as i32) != 0) as i32;
            // println!("dig {}: {:b}",&index ,quick);
            // println!("inside else q:{}",&quick);
        };
        // println!("dig out {}: {}",&index,&quick);
        if res > 1.0 {
            // println!("{}>1 res=1", &res);
            res = 1.0
        }
        if index == 0 {
            // println!("res=0 res={}", &quick);
            res = quick as f64
        }
        {
            res = abs((((quick-1+quick) as f64 *(res))) as f64+(res/2.0));
        };
    };
    if res > 1.0 {
        // println!("{}>1 res=1", &res);
        res = 1.0
    }
    return res
}
fn hexcodizer (res:f64) -> String  {
    let p = (res*15.0) as i32;
    let colour = format!("{:x}{:x}{:x}{:x}{:x}{:x}",p,p,p,p,p,p);
    return colour
}
