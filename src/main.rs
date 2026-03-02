use mathlab::functions::*;

fn main() {
    let input:u64 = 4;
    println!("i:{:b}", &input);
    let length: i32 = (floor(log2(input as f64))+1.0) as i32;
    // println!("l:{}",&length);
    let mut res:f64 = 0.0;
    calculate_one(length, input, res);
}
fn calculate_one(length:i32,input:u64,mut res:f64) {
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
        println!("{}>1 res=1", &res);
        res = 1.0
    }
    println!("ending with res:{}",&res)
}
