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

    pub objects: Vec<Object>,

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
            focus_angle: std::f32::consts::PI / 20.0,
            focus_dist: 1.475,
            objects: vec![
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
                        center: vector!(3 [0.25, -0.65, -1.35]),
                        radius: 0.35
                    }),
                    material: Material::Metal(Metal {
                        albedo: vector!(3 [0.8, 0.8, 0.8]),
                        fuzz: 0.2,
                    }),
                },
                Object {
                    geometry: Geometry::Sphere(Sphere {
                        center: vector!(3 [-0.25, -0.6, -1.6]),
                        radius: 0.4
                    }),
                    material: Material::Metal(Metal {
                        albedo: vector!(3 [0.8, 0.6, 0.2]),
                        fuzz: 0.7,
                    }),
                },
            ],

            /*
            // Simple scene with balls
            look_from: vector!(3 [-2.0, 2.0, 1.0]),
            look_at: vector!(3 [0.0, 0.0, -1.0]),
            camera_up: vector!(3 [0.0, 1.0, 0.0]),
            vfov: std::f32::consts::FRAC_PI_4,
            focal_len: 1.0,
            objects: vec![
                Object {
                    geometry: hittable::Geometry::Sphere(hittable::Sphere {
                        center: vector!(3 [0.0, 3.0, -3.0]),
                        radius: 1.0,
                    }),
                    material: material::Material::DiffuseLight(material::DiffuseLight {
                        emits: vector!(3 [4.0, 4.0, 3.6]),
                    }),
                },
                Object {
                    geometry: hittable::Geometry::Sphere(hittable::Sphere {
                        center: vector!(3 [-1.0, 0.0, -1.0]),
                        radius: 0.5,
                    }),
                    material: material::Material::Dielectric(material::Dielectric {
                        refraction_index: 1.5,
                    }),
                },
                Object {
                    geometry: hittable::Geometry::Sphere(hittable::Sphere {
                        center: vector!(3 [-1.0, 0.0, -1.0]),
                        radius: 0.4,
                    }),
                    material: material::Material::Dielectric(material::Dielectric {
                        refraction_index: 1.0 / 1.5,
                    }),
                },
                Object {
                    geometry: hittable::Geometry::Sphere(hittable::Sphere {
                        center: vector!(3 [0.0, 0.0, -1.0]),
                        radius: 0.5,
                    }),
                    material: material::Material::Lambertian(material::Lambertian {
                        albedo: vector!(3 [0.1, 0.2, 0.5]),
                    }),
                },
                Object {
                    geometry: hittable::Geometry::Sphere(hittable::Sphere {
                        center: vector!(3 [1.0, 0.0, -1.0]),
                        radius: 0.5,
                    }),
                    material: material::Material::Metal(material::Metal {
                        albedo: vector!(3 [0.8, 0.6, 0.2]),
                        fuzz: 1.0,
                    }),
                },
                Object {
                    geometry: hittable::Geometry::Plane(hittable::Plane {
                        position: vector!(3 [0.0, -0.5, 0.0]),
                        normal: vector!(3 [0.0, -1.0, 0.0]),
                    }),
                    material: material::Material::Lambertian(material::Lambertian {
                        albedo: vector!(3 [0.8, 0.8, 0.0]),
                    }),
                },
            ],
            */

            samples: 1000,
            bounces: 50,
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
                    c = c + &(self.color(ray, self.bounces) * sample_scale);
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
            let mat = &self.objects[closest_idx].material;
            let emits = mat.emits(&ray, &r);

            if let Some(s) = mat.scatter(ray, r) {
                s.attenuation * &self.color(s.scattered, depth - 1) + &emits
            } else {
                emits
            }
        } else {
            Vector::new_zeroed()
        }
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
