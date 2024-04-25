use crate::*;

pub trait Hittable where Self: Sized {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord>;
}

pub enum Geometry {
    Sphere(Sphere),
    Plane(Plane),
    Quad(Quad),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord> {
        match self {
            Self::Sphere(s) => s.hit(ray, min, max),
            Self::Plane(p) => p.hit(ray, min, max),
            Self::Quad(p) => p.hit(ray, min, max),
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
        let f = ray.direction.dot(&self.normal) < 0.0;
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

        if d <= 0.001 {
            return None;
        }

        let t = (self.position.clone() - &ray.origin).dot(&self.normal) / d;

        if t <= min || max <= t {
            return None;
        }

        let p = ray.at(t);
        let mut rec = HitRecord {
            p, normal: Matrix::new_zeroed() - &self.normal, depth: t, front_face: false };
        rec.set_out(ray);
        Some(rec)
    }
}

pub struct Quad {
    pub position: Vector<3>,
    pub u: Vector<3>,
    pub v: Vector<3>,
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, min: f32, max: f32) -> Option<HitRecord> {
        let n = self.u.cross(&self.v);
        let normal = n.clone().unit();
        let d = normal.dot(&self.position);
        let ndn = n.dot(&n);
        let w = n / ndn;

        let denom = normal.dot(&ray.direction);

        if denom <= 0.001 {
            return None;
        }

        let t = (self.position.clone() - &ray.origin).dot(&normal) / denom;

        if t <= min || max <= t {
            return None;
        }

        let p = ray.at(t);
        let hitpt = p.clone() - &self.position;
        let alpha = w.dot(&hitpt.cross(&self.v));

        if alpha < 0.0 || 1.0 < alpha {
            return None;
        }

        let beta = w.dot(&self.u.cross(&hitpt));

        if beta < 0.0 || 1.0 < beta {
            return None;
        }

        let mut rec = HitRecord {
            p, normal, depth: t, front_face: false };
        rec.set_out(ray);
        Some(rec)
    }
}
