use crate::{*, hittable::*};

pub struct ScatterInfo {
    pub scattered: Ray,
    pub attenuation: Vector<3>,
}

pub trait MaterialType {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo>;
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MaterialType for Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo> {
        match self {
            Self::Lambertian(l) => l.scatter(ray, rec),
            Self::Metal(m) => m.scatter(ray, rec),
        }
    }
}

pub struct Lambertian {
    pub albedo: Vector<3>
}

impl MaterialType for Lambertian {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo> {
        let sd = (random_unit_vec() + &rec.normal).unit();
        Some(ScatterInfo {
            scattered: Ray {
                origin: rec.p,
                direction: sd,
            },
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    pub albedo: Vector<3>
}

impl MaterialType for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo> {
        Some(ScatterInfo {
            scattered: Ray {
                origin: rec.p,
                direction: reflect(ray.direction, rec.normal),
            },
            attenuation: self.albedo.clone()
        })
    }
}

fn reflect(v: Vector<3>, n: Vector<3>) -> Vector<3> {
    let vn = v.dot(&n);
    v - &(n * vn * 2.0)
}
