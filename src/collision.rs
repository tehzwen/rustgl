use nalgebra::Vector3;

#[derive(Debug)]
pub struct BoundingBox {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}

impl BoundingBox {
    pub fn new(
        x_min: f32,
        x_max: f32,
        y_min: f32,
        y_max: f32,
        z_min: f32,
        z_max: f32,
    ) -> BoundingBox {
        BoundingBox {
            x_min,
            x_max,
            y_min,
            y_max,
            z_min,
            z_max,
        }
    }
}

pub fn get_bounding_box(vertices: &[Vector3<f32>]) -> BoundingBox {
    // Initialize min and max values
    let mut x_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;
    let mut y_min = f32::INFINITY;
    let mut y_max = f32::NEG_INFINITY;
    let mut z_min = f32::INFINITY;
    let mut z_max = f32::NEG_INFINITY;

    // Iterate over vertices
    for vertex in vertices {
        let x = vertex.x;
        let y = vertex.y;
        let z = vertex.z;

        if x > x_max {
            x_max = x;
        }
        if x < x_min {
            x_min = x;
        }
        if y > y_max {
            y_max = y;
        }
        if y < y_min {
            y_min = y;
        }
        if z > z_max {
            z_max = z;
        }
        if z < z_min {
            z_min = z;
        }
    }

    BoundingBox::new(x_min, x_max, y_min, y_max, z_min, z_max)
}
