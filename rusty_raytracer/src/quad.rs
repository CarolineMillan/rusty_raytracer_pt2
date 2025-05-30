// struct for parallelograms (the tutorial RTiOW calls them quadrilaterals)

use std::env::consts;

use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;
use nalgebra::{Normed, Point3, Vector3};
use nalgebra::RealField;

#[derive(Clone)]

pub struct Quad {
    q: Point3<f32>,
    u: Vector3<f32>,
    v: Vector3<f32>,
    w: Vector3<f32>,
    mat: Box<dyn Material + Send + Sync>,
    bbox: AABB,
    normal: Vector3<f32>,
    d: f32,
}

impl Quad {

    pub fn new(q: Point3<f32>, u: Vector3<f32>, v: Vector3<f32>, mat: Box<dyn Material>) -> Self {
        //let rvec = Vector3::new(radius, radius, radius);
        let n = u.cross(&v);
        let normal = n.normalize();
        Self {
            q,
            u,
            v,
            mat,
            bbox: set_bounding_box(q, u, v), //AABB::new_from_extrema(center - rvec, center + rvec),
            normal,
            d: q.coords.dot(&normal),
            w: n/(n.dot(&n)),
        }
    }

}

impl Hittable for Quad {

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
       
        let denom = self.normal.dot(&ray.direction());

        // no hit if the ray is parallel to the plane
        if denom.abs() < 1e-8 {
            return None;
        }
        
        let t = (self.d - self.normal.dot(&ray.origin().coords))/denom;
        if !ray_t.contains(t) {
            return None;
        }
        // does the hit point lie within the shape?
        let intersection = ray.at(t);
        let planar_hitpt_vector = intersection - self.q;
        let alpha = self.w.dot(&planar_hitpt_vector.cross(&self.v));
        let beta = self.w.dot(&self.u.cross(&planar_hitpt_vector));

        if is_interior(alpha, beta).is_none() {
            return None;
        }
        // IM NOT 100% SURE I'VE DONE THIS NORMAL CORRECTLY -- IF IT DOESN'T WORK CHECK HERE FIRST

        let mut rec = HitRecord::new_from(intersection, self.normal, self.mat.clone(), t);

        rec.u = alpha;
        rec.v = beta;
        rec.set_face_normal(ray, &self.normal);

        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        //println!("in sphere bbox");
        return self.bbox.clone();
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
}

fn set_bounding_box(q: Point3<f32>, u: Vector3<f32>, v: Vector3<f32>) -> AABB {
    let bbox_diagonal1 = AABB::new_from_extrema(q, q+u+v);
    let bbox_diagonal2 = AABB::new_from_extrema(q+u, q+v);
    return AABB::new_from_boxes(&bbox_diagonal1, &bbox_diagonal2);
}

fn is_interior(a: f32, b: f32) -> Option<(f32, f32)> {
    let unit_interval = Interval::new(0.0, 1.0);

    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        return None;
    };
    return Some((a, b));
}