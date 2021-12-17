use cv_core::FeatureMatch;
use nalgebra::{self as na, Isometry3, Matrix4, Perspective3, Point3, Vector3};
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
    pub noise: f32,
    pub outlier_proportion: f32,
    pub outlier_noise: f32,
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
            noise: 0.0,
            outlier_proportion: 0.0,
            outlier_noise: 12.0,
        }
    }
}

impl Camera {
    /// The point the camera is at
    pub fn eye(&self) -> Point3<f32> {
        Point3::new(self.x, self.y, self.z)
    }
    /// The point the camera looks at
    pub fn target(&self) -> Point3<f32> {
        Point3::new(self.target_x, self.target_y, self.target_z)
    }
    pub fn model_view_projection(&self) -> Matrix4<f32> {
        let model = Isometry3::new(Vector3::x(), na::zero());

        let mut eye = self.eye();
        // TODO find out why the view box is shifted without this
        eye.x += 1.0;
        let target = self.target();
        let view = Isometry3::look_at_rh(&eye, &target, &Vector3::y());

        // A perspective projection.
        let projection =
            Perspective3::new(self.width / self.height, self.fovy, self.znear, self.zfar);

        // The combination of the model with the view is still an isometry.
        let model_view = view * model;

        // Convert everything to a `Matrix4` so that they can be combined.
        let mat_model_view = model_view.to_homogeneous();

        // Combine everything.
        // let translation = Translation3::new(self.x, self.y, self.z).to_homogeneous();
        projection.as_matrix() * mat_model_view
    }
}

pub struct Landmark2 {
    pub id: String,
    pub point: na::Point2<f32>,
}
pub type Landmarks2 = Vec<Landmark2>;

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

pub struct MatchEvent(pub Vec<FeatureMatch<na::Point2<f64>>>);
