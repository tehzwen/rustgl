use nalgebra::{Matrix4, Point3, Vector3};

use crate::{collision::BoundingBox, scene::Scene};

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

    pub fn intersect_bounding_box(&self, bbox: &BoundingBox) -> Option<(Point3<f32>, Point3<f32>)> {
        let epsilon = 1e-6;

        let inv_dir_x = if self.direction.x.abs() > epsilon {
            1.0 / self.direction.x
        } else {
            f32::INFINITY
        };
        let inv_dir_y = if self.direction.y.abs() > epsilon {
            1.0 / self.direction.y
        } else {
            f32::INFINITY
        };
        let inv_dir_z = if self.direction.z.abs() > epsilon {
            1.0 / self.direction.z
        } else {
            f32::INFINITY
        };

        let t1 = (bbox.x_min - self.origin.x) * inv_dir_x;
        let t2 = (bbox.x_max - self.origin.x) * inv_dir_x;
        let t3 = (bbox.y_min - self.origin.y) * inv_dir_y;
        let t4 = (bbox.y_max - self.origin.y) * inv_dir_y;
        let t5 = (bbox.z_min - self.origin.z) * inv_dir_z;
        let t6 = (bbox.z_max - self.origin.z) * inv_dir_z;

        let t_min = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let t_max = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if t_min > t_max + epsilon || t_max < epsilon {
            None
        } else {
            let entry_point = self.origin + self.direction * t_min;
            let exit_point = self.origin + self.direction * t_max;
            Some((entry_point, exit_point))
        }
    }
}

pub fn ray_intersect_bb_projection(sc: &mut Scene, x: i32, y: i32) -> Option<Vector3<f32>> {
    let aspect: f32 = (sc.settings.screen_width as f32) / (sc.settings.screen_height as f32);
    let projection = Matrix4::new_perspective(aspect, sc.settings.fovy, 0.1, 10000.0);
    let main_camera = sc.cameras.get_mut(&sc.active_camera).unwrap();

    let view_projection_matrix = projection * main_camera.view_matrix();
    let inverse_vp_matrix = view_projection_matrix.try_inverse().unwrap();

    let ndc_x = (x as f32 / sc.settings.screen_width as f32) * 2.0 - 1.0;
    let ndc_y = 1.0 - (y as f32 / sc.settings.screen_height as f32) * 2.0;

    let ndc_point = Point3::new(ndc_x, ndc_y, -1.0);
    let world_point = inverse_vp_matrix.transform_point(&ndc_point);
    let ray_direction = (world_point - main_camera.position).normalize();

    let ray = Ray {
        origin: main_camera.position,
        direction: ray_direction,
    };

    let floor = sc.object_map.get_mut(&"main_plain".to_string()).unwrap();

    if let Some((entry_point, _)) = ray.intersect_bounding_box(&floor.bounding_box) {
        Some(Vector3::new(entry_point.x, entry_point.y, entry_point.z))
    } else {
        None
    }
}
