const WIDTH: usize = 300;
const HEIGHT: usize = 300;

fn main() {
    let mut fb = vec![0; WIDTH * HEIGHT * 4];
    let ray = raytracer::Raytracer::new();
    ray.render(WIDTH, HEIGHT, &mut fb);

    let mut file = Vec::new();
    file.push(0); // magic1
    file.push(0); // colormap
    file.push(2); // encoding
    file.push(0); // cmaporig and cmaplen
    file.push(0);
    file.push(0);
    file.push(0);
    file.push(0); // cmapent
    file.push(0); // x
    file.push(0);
    file.push(HEIGHT as u8); // y
    file.push((HEIGHT >> 8) as u8);
    file.push(HEIGHT as u8); // h
    file.push((HEIGHT >> 8) as u8);
    file.push(WIDTH as u8); // w
    file.push((WIDTH >> 8) as u8);
    file.push(32); // bpp
    file.push(40); // pixeltype

    for c in fb.chunks(4) {
        file.push(c[2]);
        file.push(c[1]);
        file.push(c[0]);
        file.push(c[3]);
    }

    std::fs::write("image.tga", file).unwrap();
}
