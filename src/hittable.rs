use crate::*;

pub trait Hittable where Self: Sized {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord>;
}

pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, min, max),
            Self::Plane(p) => p.hit(ray, min, max),
        }
    }
}

pub struct HitRecord {
    pub p: Vector<3>,

    pub normal: Vector<3>,
    pub depth: f32,

    pub front_face: bool,
}

impl HitRecord {
    fn set_out(&mut self, ray: &Ray) {
        let f = ray.direction.dot(&self.normal) > 0.0;
        self.front_face = f;

        if !f {
            self.normal = Matrix::new_zeroed() - &self.normal;
        }
    }
}

pub struct Sphere {
    pub center: Vector<3>,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord> {
        let oc = self.center.clone() - &ray.origin;

        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;


        let d = h * h - a * c;
        if d < 0.0 {
            return None;
        }

        let sqrtd = d.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= min || max <= root {
            root = (h + sqrtd) / a;
            if root <= min || max <= root {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p.clone() - &self.center) / self.radius;
        let mut rec = HitRecord { p, normal, depth: t, front_face: false };

        rec.set_out(ray);
        Some(rec)
    }
}

pub struct Plane {
    pub position: Vector<3>,
    pub normal: Vector<3>,
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord> {
        let d = self.normal.dot(&ray.direction);
        if d > 0.001 {
            let t = (self.position.clone() - &ray.origin).dot(&self.normal);

            if t > min && max > t {
                let p = ray.at(t);
                let mut rec = HitRecord {
                    p, normal: self.normal.clone(), depth: t, front_face: false };
                rec.set_out(ray);
                Some(rec)
            } else {
                None
            }
        } else {
            None
        }
    }
}
