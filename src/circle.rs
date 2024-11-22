use crate::framebuffer::Framebuffer;
use crate::triangle::Triangle;
use nalgebra_glm::Vec3;

pub trait Circle {
    fn circle(&mut self, center: Vec3, radius: f32);
}

impl Circle for Framebuffer {
    fn circle(&mut self, center: Vec3, radius: f32) {
        let segments = 100; // Aumenta este número para suavizar los bordes
        let angle_step = 2.0 * std::f32::consts::PI / segments as f32;

        for i in 0..segments {
            let theta1 = i as f32 * angle_step;
            let theta2 = (i + 1) as f32 * angle_step;

            let x1 = center.x + radius * theta1.cos();
            let y1 = center.y + radius * theta1.sin();
            let x2 = center.x + radius * theta2.cos();
            let y2 = center.y + radius * theta2.sin();

            let v1 = Vec3::new(x1, y1, center.z);
            let v2 = Vec3::new(x2, y2, center.z);

            // Rellenar el triángulo desde el centro al borde
            self.triangle(center, v1, v2);
        }
    }
}
