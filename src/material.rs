use crate::{*, hittable::*};

pub struct ScatterInfo {
    pub scattered: Ray,
    pub attenuation: Vector<3>,
}

pub trait MaterialType {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> ScatterInfo;
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MaterialType for Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> ScatterInfo {
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
    fn scatter(&self, _ray: Ray, rec: HitRecord) -> ScatterInfo {
        let mut sd = random_unit_vec() + &rec.normal;

        if near_zero(&sd) {
            sd = rec.normal;
        }

        ScatterInfo {
            scattered: Ray {
                origin: rec.p,
                direction: sd.unit(),
            },
            attenuation: self.albedo.clone(),
        }
    }
}

pub struct Metal {
    pub albedo: Vector<3>,
    pub fuzz: f32,
}

impl MaterialType for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> ScatterInfo {
        let reflect = reflect(ray.direction, rec.normal);
        let reflect = reflect.unit() + &(crate::random_unit_vec() * self.fuzz);

        ScatterInfo {
            scattered: Ray {
                origin: rec.p,
                direction: reflect,
            },
            attenuation: self.albedo.clone()
        }
    }
}

fn reflect(v: Vector<3>, n: Vector<3>) -> Vector<3> {
    let vn = v.dot(&n);
    v - &(n * vn * 2.0)
}

fn near_zero(v: &Vector<3>) -> bool {
    const S: f32 = 1e-6;
    v[0].abs() < S && v[1].abs() < S && v[2].abs() < S
}
