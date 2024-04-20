use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    let mut fb = rl.load_render_texture(&thread, 640, 480).unwrap();
    let mut fb_array = vec![0; 640 * 480 * 4];
    let mut state = raytracer::Raytracer::new();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        state.render(640, 480, &mut fb_array);
        fb.update_texture(&fb_array);

        d.clear_background(Color::DARKGRAY);
        d.draw_texture(&fb, 0, 0, Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_fps(0, 0);
    }
}
}
