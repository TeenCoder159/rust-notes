use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .title("Physics simulator")
        .size(800, 600)
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
