use smolmatrix::*;

pub struct Raytracer {
    pub camera: Vector<3>,
    pub focal: f32,
}

impl Raytracer {
    pub fn new() -> Self {
        Raytracer {
            camera: Vector::new_zeroed(),
            focal: 2.0,
        }
    }

    pub fn render(&self, width: usize, height: usize, fb: &[u8]) {
        macro_rules! plot {
            ($x: expr, $y: expr, $r: expr, $g: expr, $b: expr, $a: expr) => {
                {
                    let px_start = ($y * w + $x) * 4;
                    fb[px_start as usize + 0] = $r;
                    fb[px_start as usize + 1] = $g;
                    fb[px_start as usize + 2] = $b;
                    fb[px_start as usize + 3] = $a;
                }
            };
        }

        let vp_u = vector!(3 [width as f32, 0.0, 0.0]);
        let vp_v = vector!(3 [0.0, -(height as f32), 0.0]);
        let uv_dx = vp_u.clone() / width as f32;
        let uv_dy = vp_v.clone() / width as f32;

        let top_left = self.camera.clone()
            - &vector!(3 [0.0, 0.0, self.focal]) - &(vp_u.clone() / 2.0) - &(vp_v.clone() / 2.0);
        let first_px = top_left.clone() + &((vp_u.clone() + &vp_v) * 0.5);

        for y in 0..height {
            for x in 0..width {
                let px_center = first_px.clone() + &(uv_dx.clone() * x as f32) + &(uv_dy.clone() * y as f32);
                let ray_dir = px_center.clone() - &self.camera;
                let ray = Ray { origin: px_center, direction: ray_dir };
            }
        }
    }
}

struct Ray {
    origin: Vector<3>,
    direction: Vector<3>,
}
