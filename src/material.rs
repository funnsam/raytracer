use crate::*;

pub struct ScatterInfo {
    pub scattered: Ray,
    pub attenuation: Vector<3>,
}

pub trait MaterialType {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo>;
    fn emits(&self, _ray: &Ray, _rec: &HitRecord) -> Vector<3> {
        Vector::new_zeroed()
    }
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    DiffuseLight(DiffuseLight),
}

impl MaterialType for Material {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo> {
        match self {
            Self::Lambertian(l) => l.scatter(ray, rec),
            Self::Metal(m) => m.scatter(ray, rec),
            Self::Dielectric(m) => m.scatter(ray, rec),
            Self::DiffuseLight(m) => m.scatter(ray, rec),
        }
    }

    fn emits(&self, ray: &Ray, rec: &HitRecord) -> Vector<3> {
        match self {
            Self::Lambertian(m) => m.emits(ray, rec),
            Self::Metal(m) => m.emits(ray, rec),
            Self::Dielectric(m) => m.emits(ray, rec),
            Self::DiffuseLight(m) => m.emits(ray, rec),
        }
    }
}

pub struct Lambertian {
    pub albedo: Vector<3>
}

impl MaterialType for Lambertian {
    fn scatter(&self, _ray: Ray, rec: HitRecord) -> Option<ScatterInfo> {
        let mut sd = random_unit_vec3() + &rec.normal;

        if near_zero(&sd) {
            sd = rec.normal;
        }

        Some(ScatterInfo {
            scattered: Ray {
                origin: rec.p,
                direction: sd.unit(),
            },
            attenuation: self.albedo.clone(),
        })
    }
}

pub struct Metal {
    pub albedo: Vector<3>,
    pub fuzz: f32,
}

impl MaterialType for Metal {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo> {
        let reflect = reflect(ray.direction, rec.normal);
        let reflect = reflect.unit() + &(crate::random_unit_vec3() * self.fuzz);

        Some(ScatterInfo {
            scattered: Ray {
                origin: rec.p,
                direction: reflect,
            },
            attenuation: self.albedo.clone()
        })
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

pub struct Dielectric {
    pub refraction_index: f32,
}

impl MaterialType for Dielectric {
    fn scatter(&self, ray: Ray, rec: HitRecord) -> Option<ScatterInfo> {
        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };
        let cos_theta = (Matrix::new_zeroed() - &ray.direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        Some(if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > rand::random() {
            ScatterInfo {
                scattered: Ray { origin: rec.p, direction: reflect(ray.direction, rec.normal).unit() },
                attenuation: vector!(3 [1.0, 1.0, 1.0]),
            }
        } else {
            ScatterInfo {
                scattered: Ray { origin: rec.p, direction: refract(ray.direction, rec.normal, ri).unit() },
                attenuation: vector!(3 [1.0, 1.0, 1.0]),
            }
        })
    }
}

fn refract(uv: Vector<3>, n: Vector<3>, etai_ov_etat: f32) -> Vector<3> {
    let cos_theta = (Matrix::new_zeroed() - &uv).dot(&n).min(1.0);

    let r_out_perp = (n.clone() * cos_theta + &uv) * etai_ov_etat;
    let r_out_parl = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();

    r_out_perp + &r_out_parl
}

fn reflectance(cos_theta: f32, ri: f32) -> f32 {
    let r0 = (1.0 - ri) / (1.0 + ri);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}

pub struct DiffuseLight {
    pub emits: Vector<3>,
}

impl MaterialType for DiffuseLight {
    fn scatter(&self, _ray: Ray, _rec: HitRecord) -> Option<ScatterInfo> {
        None
    }

    fn emits(&self, _ray: &Ray, _rec: &HitRecord) -> Vector<3> {
        self.emits.clone()
    }
}
