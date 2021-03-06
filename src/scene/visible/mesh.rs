use crate::common::{Intersection, Ray, Spacial, Vec3, VertexFormat};
use crate::scene::visible::{Intersectable, Visible};

pub struct Triangle<T: VertexFormat> {
    vertices: Vec<Vec3<T>>,
    // Triangle normal
    normal: Vec3<T>,
    // distance to origin
    d: T,
}

impl<T: VertexFormat> Triangle<T> {
    pub fn new(v1: Vec3<T>, v2: Vec3<T>, v3: Vec3<T>) -> Triangle<T> {
        let vector1 = v2.sub(&v1);
        let vector2 = v3.sub(&v1);

        let normal = vector1.cross(&vector2).normalize();

        let d = Vec3::new(T::zero(), T::zero(), T::zero())
            .sub(&v1)
            .dot(&normal);

        Triangle {
            vertices: vec![v1, v2, v3],
            normal,
            d,
        }
    }

    fn axis_to_drop(&self) -> u8 {
        if self.normal.x >= self.normal.y && self.normal.x >= self.normal.z {
            0
        } else if self.normal.y >= self.normal.x && self.normal.y >= self.normal.z {
            1
        } else {
            2
        }
    }

    fn project(&self, vector: &Vec3<T>, axis_to_drop: u8) -> (T, T) {
        if axis_to_drop == 0 {
            (vector.y, vector.z)
        } else if axis_to_drop == 1 {
            (vector.x, vector.z)
        } else {
            (vector.x, vector.y)
        }
    }

    fn projection_intersection(&self, plane_intersection: &Vec3<T>) -> bool {
        let axis_to_drop = self.axis_to_drop();
        let mut uv_vector = Vec::new();
        uv_vector.push(self.project(&self.vertices[0].sub(plane_intersection), axis_to_drop));
        uv_vector.push(self.project(&self.vertices[1].sub(plane_intersection), axis_to_drop));
        uv_vector.push(self.project(&self.vertices[2].sub(plane_intersection), axis_to_drop));

        let mut sign_holder: i8;
        let mut next_sign_holder: i8;

        let mut num_crossings = 0;

        if uv_vector.first().unwrap().1 < T::zero() {
            sign_holder = -1;
        } else {
            sign_holder = 1;
        }

        for i in 0..uv_vector.len() {
            let i_plus = (i + 1) % uv_vector.len();

            let uv = uv_vector[i];
            let uv_plus = uv_vector[i_plus];

            if uv_plus.1 < T::zero() {
                next_sign_holder = -1;
            } else {
                next_sign_holder = 1;
            }

            if sign_holder != next_sign_holder {
                if (uv.0 > T::zero() && uv_plus.0 > T::zero()) {
                    num_crossings += 1;
                } else if uv.0 > T::zero() || uv_plus.0 > T::zero() {
                    let u_cross = uv.0 - uv.1 * (uv_plus.0 - uv.0) / (uv_plus.1 - uv.1);
                    if u_cross > T::zero() {
                        num_crossings += 1;
                    }
                }
            }

            sign_holder = next_sign_holder;
        }

        num_crossings % 2 == 1
    }
}

impl<T: VertexFormat> Intersectable<T> for Triangle<T> {
    fn intersect(&self, ray: &Ray<T>) -> Option<Intersection<T>> {
        let v_d = self.normal.dot(&ray.direction);

        if v_d == T::zero() {
            return None;
        }
        //     if one sided plane, uncomment
        // else if v_d > T::zero() {
        //     return None;
        // }

        let t = -(self.normal.dot(&ray.origin) + self.d) / v_d;
        if t < T::zero() {
            return None;
        }
        let mut normal = self.normal.clone();

        // comment out if one sided planes
        if v_d > T::zero() {
            normal = normal.mul(T::one().neg());
        }

        let intersection_point = ray.origin.add(&ray.direction.mul(t));

        if self.projection_intersection(&intersection_point) {
            Some(Intersection {
                point: intersection_point,
                normal,
            })
        } else {
            None
        }
    }
}

impl<T: VertexFormat> Spacial<T> for Triangle<T> {
    fn location(&self) -> &Vec3<T> {
        //     TODO: maybe change this to be the something else? like an object origin or something.
        &self.vertices.first().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_plane_intersection() {
        let v1 = Vec3::new(-1.0, -1.0, -1.0);
        let v2 = Vec3::new(1.0, -1.0, -1.0);
        let v3 = Vec3::new(0.0, 1.0, -1.0);

        let mesh = Triangle::new(v1, v2, v3);

        let ray = Ray::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, -1.0));

        let exptected_intersection = Intersection {
            point: Vec3::new(0.0, 0.0, -1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
        };

        assert_eq!(exptected_intersection, mesh.intersect(&ray).unwrap());
    }

    #[test]
    fn ray_plane_miss() {
        let v1 = Vec3::new(-1.0, -1.0, -1.0);
        let v2 = Vec3::new(1.0, -1.0, -1.0);
        let v3 = Vec3::new(0.0, 1.0, -1.0);

        let mesh = Triangle::new(v1, v2, v3);

        let ray = Ray::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0));

        assert_eq!(None, mesh.intersect(&ray));
    }
}
