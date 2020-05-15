use learn_traits_draw::{Screen, Button, SelectBox};
use std::boxed::Box;
use std::slice;

fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components: vec![
            Box::new(Button { width: 1, height: 2, label: String::from("hello!")}),
            Box::new(SelectBox { options: Vec::new() }),
        ],
    };

    screen.run();

    let mut arr = [1, 2, 3];

    let (larr, rarr) = split_at_mut(&mut arr, 1);

    println!("Got: {:?} and {:?}", larr, rarr);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len-mid),
        )
    }
}
