use std::{cmp::min, fs::read_to_string};

// NOTE: I want to improve typechecks as well, reduce the use of unwrap()
// but for now, let's move on

// did't use the concept of global contants in this case, will do later
const LI: u8 = 0;
const BI: u8 = 1;
const HI: u8 = 2;

fn main() {
    let input = read_to_string("data/3.txt").unwrap();

    let boxes: Vec<&str> = input.split("\n").collect();

    let mut i = 0;
    let mut total_area = 0;
    for i in 0..boxes.len() - 1 {
        let curr_box_result = boxes.get(i);

        if let Some(curr_box) = curr_box_result {
            let dimentions: Vec<&str> = curr_box.split("x").collect();
            let l: i32 = dimentions.get(0).unwrap().parse().unwrap();
            let b: i32 = dimentions.get(1).unwrap().parse().unwrap();
            let h: i32 = dimentions.get(2).unwrap().parse().unwrap();

            println!("Processing... L: {}, B: {}, H: {}", l, b, h);

            let area_1 = l * b;
            let area_2 = b * h;
            let area_3 = h * l;

            let minimum = min(area_3, min(area_1, area_2));
            let area = (2 * (area_1 + area_2 + area_3)) + minimum;

            total_area += area;
            println!("Area: {}, Smallest: {}", area, minimum);
        }
    }

    println!("Total area needed: {}", total_area);
}
