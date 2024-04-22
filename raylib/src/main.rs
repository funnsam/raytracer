use raylib::prelude::*;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Hello, World")
        .build();

    let mut fb = rl.load_render_texture(&thread, WIDTH as u32, HEIGHT as u32).unwrap();
    let mut fb_array = vec![0; WIDTH * HEIGHT * 4];
    let mut state = raytracer::Raytracer::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        state.render(WIDTH, HEIGHT, &mut fb_array);
        fb.update_texture(&fb_array);

        d.clear_background(Color::DARKGRAY);
        d.draw_texture(&fb, 0, 0, Color::WHITE);
    }
}
