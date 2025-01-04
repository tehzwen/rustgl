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

pub fn get_bounding_box(vertices: &[f32]) -> BoundingBox {
    // Initialize min and max values
    let mut x_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;
    let mut y_min = f32::INFINITY;
    let mut y_max = f32::NEG_INFINITY;
    let mut z_min = f32::INFINITY;
    let mut z_max = f32::NEG_INFINITY;

    // Iterate over vertices in steps of 3 (x, y, z)
    for chunk in vertices.chunks_exact(3) {
        let x = chunk[0];
        let y = chunk[1];
        let z = chunk[2];

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

    return BoundingBox::new(x_min, x_max, y_min, y_max, z_min, z_max);
}
