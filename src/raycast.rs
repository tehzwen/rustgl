use nalgebra::{Point3, Vector3};

use crate::collision::BoundingBox;

pub struct Ray {
    pub origin: Point3<f32>,
    pub direction: Vector3<f32>,
}

struct Plane {
    point: Point3<f32>,
    normal: Vector3<f32>,
}

impl Ray {
    pub fn intersect_plane(&self, plane: &Plane) -> Option<Point3<f32>> {
        // Calculate the denominator (dot product of ray direction and plane normal)
        let denom = plane.normal.dot(&self.direction);

        // If denominator is 0, the ray is parallel to the plane, so no intersection
        if denom.abs() < f32::EPSILON {
            return None;
        }

        // Calculate the t value for the intersection point
        let t = plane.normal.dot(&(plane.point - self.origin)) / denom;

        // If t is negative, the intersection point is behind the ray origin
        if t < 0.0 {
            return None;
        }

        // Calculate the intersection point
        let intersection = self.origin + self.direction * t;

        Some(intersection)
    }

    /// Checks if the ray intersects with a bounding box.
    ///
    /// Returns the t_min and t_max values if there's an intersection.
    pub fn intersect_bounding_box(&self, bbox: &BoundingBox) -> Option<(Point3<f32>, Point3<f32>)> {
        let inv_dir = Vector3::new(
            1.0 / self.direction.x,
            1.0 / self.direction.y,
            1.0 / self.direction.z,
        );

        let t1 = (bbox.x_min - self.origin.x) * inv_dir.x;
        let t2 = (bbox.x_max - self.origin.x) * inv_dir.x;
        let t3 = (bbox.y_min - self.origin.y) * inv_dir.y;
        let t4 = (bbox.y_max - self.origin.y) * inv_dir.y;
        let t5 = (bbox.z_min - self.origin.z) * inv_dir.z;
        let t6 = (bbox.z_max - self.origin.z) * inv_dir.z;

        let t_min = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let t_max = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        // If t_min > t_max, the ray misses the bounding box
        if t_min > t_max || t_max < 0.0 {
            None
        } else {
            let entry_point = self.origin + self.direction * t_min;
            let exit_point = self.origin + self.direction * t_max;
            Some((entry_point, exit_point))
        }
    }
}
