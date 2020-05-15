use learn_traits_draw::{Screen, Button, SelectBox};
use std::boxed::Box;

fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components: vec![
            Box::new(Button { width: 1, height: 2, label: String::from("hello!")}),
            Box::new(SelectBox { options: Vec::new() }),
        ],
    };

    screen.run();
}
