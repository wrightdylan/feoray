use crate::core::{canvas, point, Canvas, Ray, World};
use nalgebra::Matrix4;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub fov: f64,
    pub px_size: f64,
    pub transform: Matrix4<f64>,
    half_width: f64,
    half_height:f64
}

impl Camera {
    /// Initialise new camera.
    pub fn new(hsize: usize, vsize: usize, fov: f64) -> Self {
        let half_view = (fov/2.0).tan();
        let aspect = hsize as f64/vsize as f64;
        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };
        let px_size = (half_width * 2.0) / hsize as f64;
        let transform = Matrix4::identity();
        Self {
            hsize,
            vsize,
            fov,
            px_size,
            transform,
            half_width,
            half_height
        }
    }

    /// Creates a single ray for the specified pixel.
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let world_x = self.half_width - (px as f64 + 0.5) * self.px_size;
        let world_y = self.half_height - (py as f64 + 0.5) * self.px_size;
        let mut pixel = self.transform.try_inverse().unwrap() * point(world_x, world_y, -1.0);
        let mut origin = self.transform.try_inverse().unwrap() * point(0.0, 0.0, 0.0);
        pixel.w = 1.0; // on second thought, assigning the correction may be easier on memory than to_point()
        origin.w = 1.0;
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    /// Routine to render a scene to a canvas. Canvas can then be exported to
    /// an image file.
    pub fn render(&self, world: World) -> Canvas {
        let mut canvas = canvas(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let colour = world.colour_at(&ray, 1);
                canvas.write_pix(x, y, colour);
            }
        }

        canvas
    }

    /// Applies a transform directly to the camera. The only transform that should be
    /// applied is view_transform().
    pub fn with_transform(&mut self, transform: Matrix4<f64>) -> Self {
        self.transform = transform;

        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{vector, Colour, Transform, Tuple, World};
    use std::f64::consts::PI;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let fov = PI/2.0;
        let cam = Camera::new(hsize, vsize, fov);

        assert_eq!(cam.hsize, 160);
        assert_eq!(cam.vsize, 120);
        assert_eq!(cam.fov, PI/2.0);
        assert_eq!(cam.transform, Matrix4::identity());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let cam = Camera::new(200, 125, PI/2.0);

        assert_eq!((cam.px_size * 100.0).round() / 100.0, 0.01);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let cam = Camera::new(125, 200, PI/2.0);

        assert_eq!((cam.px_size * 100.0).round() / 100.0, 0.01);
    }

    #[test]
    fn constructing_ray_through_centre_of_canvas() {
        let cam = Camera::new(201, 101, PI/2.0);
        let r = cam.ray_for_pixel(100, 50);

        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction.to_5dp(), vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_ray_through_corner_of_canvas() {
        let cam = Camera::new(201, 101, PI/2.0);
        let r = cam.ray_for_pixel(0, 0);

        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction.to_5dp(), vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_ray_when_camera_transformed() {
        let mut cam = Camera::new(201, 101, PI/2.0);
        let t = Matrix4::rot_y(PI/4.0) * Matrix4::translate(0.0, -2.0, 5.0);
        cam.with_transform(t);
        let r = cam.ray_for_pixel(100, 50);
        let irr_no = 2.0f64.sqrt() / 2.0;

        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(r.direction.to_5dp(), vector(irr_no, 0.0, -irr_no).to_5dp());
    }

    #[test]
    fn rendering_world_with_camera() {
        let w = World::default_world();
        let mut cam = Camera::new(11, 11, PI/2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        cam.with_transform(Matrix4::view_transform(from, to, up));
        let image = cam.render(w);

        assert_eq!(image.read_pix(5, 5).to_5dp(), Colour::new(0.38066, 0.47583, 0.2855));
    }
}