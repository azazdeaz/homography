use nalgebra as na;
pub struct Camera {
    pub width: f32,
    pub height: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub target_x: f32,
    pub target_y: f32,
    pub target_z: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            width: 300.0,
            height: 200.0,
            fovy: 1.9,
            znear: 1.0,
            zfar: 100.0,
            x: 0.0,
            y: 0.0,
            z: 1.0,
            target_x: 0.0,
            target_y: 0.0,
            target_z: 0.0,
        }
    }
}

pub struct Landmark2 {
    pub id: String,
    pub point: na::Point2<f32>,
}

pub struct Landmark3 {
    pub id: String,
    pub point: na::Point3<f32>,
}
pub type Landmarks3 = Vec<Landmark3>;

pub struct Plane {
    pub id: usize,
    pub points_x: u32,
    pub points_y: u32,
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
}
