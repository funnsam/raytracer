#![allow(mutable_transmutes)]

use smolmatrix::*;
use rayon::prelude::*;

pub mod hittable;
use hittable::*;
pub mod material;
use material::*;

pub struct Object {
    pub geometry: hittable::Geometry,
    pub material: material::Material,
}

pub struct Raytracer {
    pub look_from: Vector<3>,
    pub look_at: Vector<3>,
    pub camera_up: Vector<3>,

    pub vfov: f32,

    pub focus_angle: f32,
    pub focus_dist: f32,

    pub world: World,
    pub lights: Vec<Vector<3>>,

    pub samples: usize,
    pub bounces: usize,
}

impl Raytracer {
    pub fn new() -> Self {
        Raytracer {
            // Cornell box
            look_from: vector!(3 [0.0, 0.0, 0.0]),
            look_at: vector!(3 [0.0, 0.0, -1.0]),
            camera_up: vector!(3 [0.0, 1.0, 0.0]),
            vfov: std::f32::consts::FRAC_PI_2,
            focus_angle: 0.0, // std::f32::consts::PI / 20.0,
            focus_dist: 1.60857079421,
            world: World(vec![
                Object {
                    geometry: Geometry::Plane(Plane {
                        position: vector!(3 [0.0, -1.0, 0.0]),
                        normal: vector!(3 [0.0, -1.0, 0.0]),
                    }),
                    material: Material::Lambertian(Lambertian {
                        albedo: vector!(3 [0.73, 0.73, 0.73]),
                    }),
                },
                Object {
                    geometry: Geometry::Plane(Plane {
                        position: vector!(3 [0.0, 1.0, 0.0]),
                        normal: vector!(3 [0.0, 1.0, 0.0]),
                    }),
                    material: Material::Lambertian(Lambertian {
                        albedo: vector!(3 [0.73, 0.73, 0.73]),
                    }),
                },
                Object {
                    geometry: Geometry::Plane(Plane {
                        position: vector!(3 [0.0, 0.0, -2.0]),
                        normal: vector!(3 [0.0, 0.0, -1.0]),
                    }),
                    material: Material::Lambertian(Lambertian {
                        albedo: vector!(3 [0.73, 0.73, 0.73]),
                    }),
                },
                Object {
                    geometry: Geometry::Plane(Plane {
                        position: vector!(3 [1.0, 0.0, 0.0]),
                        normal: vector!(3 [1.0, 0.0, 0.0]),
                    }),
                    material: Material::Lambertian(Lambertian {
                        albedo: vector!(3 [0.65, 0.05, 0.05]),
                    }),
                },
                Object {
                    geometry: Geometry::Plane(Plane {
                        position: vector!(3 [-1.0, 0.0, 0.0]),
                        normal: vector!(3 [-1.0, 0.0, 0.0]),
                    }),
                    material: Material::Lambertian(Lambertian {
                        albedo: vector!(3 [0.12, 0.45, 0.15]),
                    }),
                },
                Object {
                    geometry: Geometry::Quad(Quad {
                        position: vector!(3 [-0.5, 0.99, -0.5]),
                        u: vector!(3 [0.0, 0.0, -1.0]),
                        v: vector!(3 [1.0, 0.0, 0.0]),
                    }),
                    material: Material::DiffuseLight(DiffuseLight {
                        emits: vector!(3 [4.0, 4.0, 4.0]),
                    }),
                },
                // Cornell box objects
                Object {
                    geometry: Geometry::Sphere(Sphere {
                        center: vector!(3 [0.25, -0.05, -1.4]),
                        radius: 0.25
                    }),
                    material: Material::Dielectric(Dielectric {
                        attenuation: vector!(3 [0.8, 0.8, 1.0]),
                        refraction_index: 2.417,
                    }),
                },
                Object {
                    geometry: Geometry::Sphere(Sphere {
                        center: vector!(3 [-0.25, -0.4, -1.45]),
                        radius: 0.25
                    }),
                    material: Material::Metal(Metal {
                        albedo: vector!(3 [0.72, 0.45, 0.2]),
                        fuzz: 0.4,
                    }),
                },
            ]),

            lights: vec![
                vector!(3 [0.0, 0.99, -1.0]),
            ],

            samples: 16,
            bounces: 20,
        }
    }

    pub fn render(&self, width: usize, height: usize, fb: &mut [u8]) {
        let sample_scale = 1.0 / self.samples as f32;

        let h = (self.vfov / 2.0).tan();

        let vp_h = 2.0 * h * self.focus_dist;
        let vp_w = vp_h * (width as f32 / height as f32);

        let w = (self.look_from.clone() - &self.look_at).unit();
        let u = self.camera_up.cross(&w).unit();
        let v = w.cross(&u);

        let vp_u = u.clone() * vp_w;
        let vp_v = v.clone() * -vp_h;

        let uv_dx = vp_u.clone() / width as f32;
        let uv_dy = vp_v.clone() / height as f32;

        let top_left = self.look_from.clone()
            - &(w * self.focus_dist)
            - &(vp_u.clone() / 2.0)
            - &(vp_v.clone() / 2.0);
        let first_px = top_left.clone() + &((uv_dx.clone() + &uv_dy) * 0.5);

        (0..height).into_par_iter().for_each(|y| {
            let fb = unsafe { core::mem::transmute::<&_, &mut [u8]>(fb) };

            let defoc_rad = self.focus_dist * (self.focus_angle / 2.0).tan();
            let defoc_u = u.clone() * defoc_rad;
            let defoc_v = v.clone() * defoc_rad;

            for x in 0..width {
                let mut c = Matrix::new_zeroed();

                for _ in 0..self.samples {
                    let sample = first_px.clone()
                        + &(uv_dx.clone() * (x as f32 + rand::random::<f32>() - 0.5))
                        + &(uv_dy.clone() * (y as f32 + rand::random::<f32>() - 0.5));
                    let origin = self.defoc_sample(defoc_u.clone(), defoc_v.clone());
                    let ray_dir = sample.clone() - &origin;
                    let ray = Ray { origin, direction: ray_dir.unit() };
                    c = c + &(self.color(ray) * sample_scale);
                }

                fn gamma_corr(c: f32) -> f32 {
                    c.sqrt()
                }

                let px_start = (y * width + x) * 4;
                fb[px_start + 0] = (gamma_corr(c[0].max(0.0).min(1.0)) * 255.0) as u8;
                fb[px_start + 1] = (gamma_corr(c[1].max(0.0).min(1.0)) * 255.0) as u8;
                fb[px_start + 2] = (gamma_corr(c[2].max(0.0).min(1.0)) * 255.0) as u8;
                fb[px_start + 3] = 255;
            }

            #[cfg(feature = "report_progress")]
            println!("row {y} done");
        });
    }

    fn defoc_sample(&self, du: Vector<3>, dv: Vector<3>) -> Vector<3> {
        let d = random_unit_vec2();
        self.look_from.clone() + &(du * d[0]) + &(dv * d[1])
    }

    fn color(&self, mut ray: Ray) -> Vector<3> {
        let mut color = Matrix::new_zeroed();
        let mut throughput = vector!(3 [1.0, 1.0, 1.0]);

        for i in 0..self.bounces {
            if let Some((r, idx)) = self.world.hit(&ray, 0.001, f32::INFINITY) {
                let mat = &self.world.0[idx].material;
                let mut emits = mat.emits(&ray, &r);

                // russian roulette
                let p = throughput[0].max(throughput[1]).max(throughput[2]);
                if rand::random::<f32>() > p {
                    break;
                }

                throughput = throughput / p;

                // nee optimization
                if let Some(sl_at) = self.get_sample_light() {
                    let sl_dir = (sl_at.clone() - &r.p).unit();
                    let sl_ray = Ray { origin: r.p.clone(), direction: sl_dir };

                    if let Some(sl_emits) = self.world.hit(&sl_ray, 0.01, f32::INFINITY).map(|(r, i)| self.world.0[i].material.emits(&sl_ray, &r)) {
                        emits = (emits + &sl_emits) * if i == 0 { 0.5 } else { 1.0 };
                    }
                }

                if let Some(s) = mat.scatter(ray, r) {
                    throughput = throughput * &s.attenuation;

                    ray = s.scattered;
                    color = color + &(emits * &throughput);
                } else {
                    color = color + &(emits * &throughput);
                    break;
                }
            } else {
                break;
            }
        }

        color
    }

    fn get_sample_light(&self) -> Option<&Vector<3>> {
        // None
        Some(&self.lights[rand::random::<usize>() % self.lights.len()])
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

fn random_vec3() -> Vector<3> {
    vector!(3 [
        rand::random::<f32>() * 2.0 - 1.0,
        rand::random::<f32>() * 2.0 - 1.0,
        rand::random::<f32>() * 2.0 - 1.0,
    ])
}

fn random_unit_vec3() -> Vector<3> {
    loop {
        let v = random_vec3();
        if v.length_squared() < 1.0 {
            return v.unit();
        }
    }
}

fn random_vec2() -> Vector<3> {
    vector!(3 [
        rand::random::<f32>() * 2.0 - 1.0,
        rand::random::<f32>() * 2.0 - 1.0,
        0.0,
    ])
}

fn random_unit_vec2() -> Vector<3> {
    loop {
        let v = random_vec2();
        if v.length_squared() < 1.0 {
            return v.unit();
        }
    }
}
