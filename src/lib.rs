use smolmatrix::*;

pub mod hittable;
pub mod material;

pub struct Object {
    pub geometry: hittable::Geometry,
    pub material: material::Material,
}

pub struct Raytracer {
    pub camera: Vector<3>,
    pub focal: f32,

    pub objects: Vec<Object>,

    pub samples: usize,
    pub bounces: usize,
}

impl Raytracer {
    pub fn new() -> Self {
        Raytracer {
            camera: Vector::new_zeroed(),
            focal: 1.0,
            objects: vec![
                Object {
                    geometry: hittable::Geometry::Sphere(hittable::Sphere {
                        center: vector!(3 [0.5, 0.0, -1.0]),
                        radius: 0.5,
                    }),
                    material: material::Material::Metal(material::Metal {
                        albedo: vector!(3 [1.0, 0.0, 0.0]),
                    }),
                },
                Object {
                    geometry: hittable::Geometry::Sphere(hittable::Sphere {
                        center: vector!(3 [-0.5, 1.0, -3.0]),
                        radius: 1.0,
                    }),
                    material: material::Material::Lambertian(material::Lambertian {
                        albedo: vector!(3 [0.0, 1.0, 0.0]),
                    }),
                },
                Object {
                    geometry: hittable::Geometry::Plane(hittable::Plane {
                        position: vector!(3 [0.0, -1.0, 0.0]),
                        normal: vector!(3 [0.0, -1.0, 0.0]),
                    }),
                    material: material::Material::Lambertian(material::Lambertian {
                        albedo: vector!(3 [0.0, 0.0, 1.0]),
                    }),
                },
            ],

            samples: 25,
            bounces: 20,
        }
    }

    pub fn render(&self, width: usize, height: usize, fb: &mut [u8]) {
        let sample_scale = 1.0 / self.samples as f32;

        let vp_h = 2.0;
        let vp_w = vp_h * (width as f32 / height as f32);

        let vp_u = vector!(3 [vp_w, 0.0, 0.0]);
        let vp_v = vector!(3 [0.0, -vp_h, 0.0]);

        let uv_dx = vp_u.clone() / width as f32;
        let uv_dy = vp_v.clone() / height as f32;

        let top_left = self.camera.clone()
            - &vector!(3 [0.0, 0.0, self.focal]) - &(vp_u.clone() / 2.0) - &(vp_v.clone() / 2.0);
        let first_px = top_left.clone() + &((uv_dx.clone() + &uv_dy) * 0.5);

        for y in 0..height {
            for x in 0..width {
                let mut c = Matrix::new_zeroed();

                for _ in 0..self.samples {
                    let px_center = first_px.clone()
                        + &(uv_dx.clone() * (x as f32 + rand::random::<f32>() - 0.5))
                        + &(uv_dy.clone() * (y as f32 + rand::random::<f32>() - 0.5));
                    let ray_dir = px_center.clone() - &self.camera;
                    let ray = Ray { origin: px_center, direction: ray_dir.unit() };
                    c = c + &(self.color(ray, self.bounces) * sample_scale);
                }

                fn gamma_corr(c: f32) -> f32 {
                    if c > 0.0 { c.sqrt() } else { 0.0 }
                }

                let px_start = (y * width + x) * 4;
                fb[px_start + 0] = (gamma_corr(c[0].min(1.0)) * 255.0) as u8;
                fb[px_start + 1] = (gamma_corr(c[1].min(1.0)) * 255.0) as u8;
                fb[px_start + 2] = (gamma_corr(c[2].min(1.0)) * 255.0) as u8;
                fb[px_start + 3] = 255;
            }

            #[cfg(feature = "report_progress")]
            println!("row {y} done");
        }
    }

    fn color(&self, ray: Ray, depth: usize) -> Vector<3> {
        if depth == 0 {
            return Matrix::new_zeroed();
        }

        let mut closest_dst = f32::INFINITY;
        let mut closest_rec = None;
        let mut closest_idx = 0;
        for (i, obj) in self.objects.iter().enumerate() {
            use hittable::Hittable;
            if let Some(r) = obj.geometry.hit(&ray, 0.001, closest_dst) {
                closest_dst = r.depth;
                closest_rec = Some(r);
                closest_idx = i;
            }
        }

        if let Some(r) = closest_rec {
            use material::MaterialType;

            if let Some(s) = self.objects[closest_idx].material.scatter(ray, r) {
                return s.attenuation * &self.color(s.scattered, depth - 1);
            } else {
                return Matrix::new_zeroed();
            }
        }

        let a = 0.5 * (ray.direction[1] + 1.0);
        vector!(3 [1.0, 1.0, 1.0]) * (1.0 - a) + &(vector!(3 [0.5, 0.7, 1.0]) * a)
    }
}

#[derive(Clone)]
pub struct Ray {
    origin: Vector<3>,
    direction: Vector<3>,
}

impl Ray {
    pub fn at(&self, p: f32) -> Vector<3> {
        self.origin.clone() + &(self.direction.clone() * p)
    }
}

fn random_vec() -> Vector<3> {
    vector!(3 [
        rand::random::<f32>() * 2.0 - 1.0,
        rand::random::<f32>() * 2.0 - 1.0,
        rand::random::<f32>() * 2.0 - 1.0,
    ])
}

fn random_sphere_vec() -> Vector<3> {
    loop {
        let v = random_vec();
        if v.length_squared() < 1.0 {
            return v;
        }
    }
}

fn random_unit_vec() -> Vector<3> {
    random_sphere_vec().unit()
}

fn random_hemisphere(n: &Vector<3>) -> Vector<3> {
    let s = random_unit_vec();

    if s.dot(n) < 0.0 { s } else { Matrix::new_zeroed() - &s }
}
