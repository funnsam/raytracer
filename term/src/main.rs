#![feature(iter_array_chunks)]

fn main() {
    let (w, h) = term_size::dimensions().unwrap();
    let w = w as usize;
    let h = h as usize * 2;

    let mut fb = vec![0; w * h * 4];
    raytracer::Raytracer::new().render(w, h, &mut fb);

    draw(&fb, w);
}

fn draw(fb: &[u8], w: usize) {
    let pixel = fb.chunks(4).collect::<Vec<&[u8]>>();
    let row = pixel.chunks(w);

    for [a, b] in row.array_chunks() {
        for (a, b) in a.iter().zip(b.iter()) {
            plot(a, b);
        }

        println!("\x1b[0m");
    }
}

fn plot(a: &[u8], b: &[u8]) {
    print!("\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀", a[0], a[1], a[2], b[0], b[1], b[2]);
}
