//! 3D Surface Plot Widget
//!
//! Displays 3D surface data with perspective projection and color mapping.
//! GPU-accelerated with rotation animation and proper depth sorting.

use makepad_widgets::*;
use std::f64::consts::PI;
use super::draw_primitives::{DrawChartLine, DrawTriangle};
use super::animation::{ChartAnimator, EasingType};

live_design! {
    link widgets;
    use link::shaders::*;
    use super::draw_primitives::DrawChartLine;
    use super::draw_primitives::DrawTriangle;

    pub SurfacePlotWidget = {{SurfacePlotWidget}} {
        width: Fill,
        height: Fill,
    }
}

/// Color map types for surface visualization
#[derive(Clone, Copy, Debug, Default)]
pub enum ColorMap {
    #[default]
    Viridis,
    Plasma,
    Inferno,
    Magma,
    CoolWarm,
    Terrain,
    Rainbow,
}

impl ColorMap {
    pub fn get_color(&self, t: f64) -> Vec4 {
        let t = t.clamp(0.0, 1.0) as f32;

        match self {
            ColorMap::Viridis => {
                if t < 0.25 {
                    let s = t / 0.25;
                    vec4(0.267, 0.004 + s * 0.25, 0.329 + s * 0.15, 1.0)
                } else if t < 0.5 {
                    let s = (t - 0.25) / 0.25;
                    vec4(0.267 + s * 0.02, 0.254 + s * 0.25, 0.479 - s * 0.05, 1.0)
                } else if t < 0.75 {
                    let s = (t - 0.5) / 0.25;
                    vec4(0.287 + s * 0.2, 0.504 + s * 0.2, 0.429 - s * 0.15, 1.0)
                } else {
                    let s = (t - 0.75) / 0.25;
                    vec4(0.487 + s * 0.45, 0.704 + s * 0.15, 0.279 - s * 0.1, 1.0)
                }
            }
            ColorMap::Plasma => {
                if t < 0.33 {
                    let s = t / 0.33;
                    vec4(0.05 + s * 0.45, 0.03 + s * 0.02, 0.53 + s * 0.2, 1.0)
                } else if t < 0.66 {
                    let s = (t - 0.33) / 0.33;
                    vec4(0.5 + s * 0.35, 0.05 + s * 0.25, 0.73 - s * 0.35, 1.0)
                } else {
                    let s = (t - 0.66) / 0.34;
                    vec4(0.85 + s * 0.1, 0.3 + s * 0.55, 0.38 - s * 0.35, 1.0)
                }
            }
            ColorMap::Inferno => {
                if t < 0.33 {
                    let s = t / 0.33;
                    vec4(0.0 + s * 0.25, 0.0 + s * 0.05, 0.01 + s * 0.35, 1.0)
                } else if t < 0.66 {
                    let s = (t - 0.33) / 0.33;
                    vec4(0.25 + s * 0.55, 0.05 + s * 0.1, 0.36 - s * 0.15, 1.0)
                } else {
                    let s = (t - 0.66) / 0.34;
                    vec4(0.8 + s * 0.18, 0.15 + s * 0.75, 0.21 - s * 0.2, 1.0)
                }
            }
            ColorMap::Magma => {
                if t < 0.33 {
                    let s = t / 0.33;
                    vec4(0.0 + s * 0.2, 0.0 + s * 0.03, 0.02 + s * 0.3, 1.0)
                } else if t < 0.66 {
                    let s = (t - 0.33) / 0.33;
                    vec4(0.2 + s * 0.55, 0.03 + s * 0.15, 0.32 + s * 0.1, 1.0)
                } else {
                    let s = (t - 0.66) / 0.34;
                    vec4(0.75 + s * 0.23, 0.18 + s * 0.7, 0.42 + s * 0.2, 1.0)
                }
            }
            ColorMap::CoolWarm => {
                // Blue (cold) to white to red (hot)
                if t < 0.5 {
                    let s = t / 0.5;
                    vec4(0.2 + s * 0.8, 0.4 + s * 0.6, 0.9 + s * 0.1, 1.0)
                } else {
                    let s = (t - 0.5) / 0.5;
                    vec4(1.0, 1.0 - s * 0.6, 1.0 - s * 0.8, 1.0)
                }
            }
            ColorMap::Terrain => {
                // Deep blue -> green -> yellow -> brown -> white
                if t < 0.2 {
                    let s = t / 0.2;
                    vec4(0.2, 0.3 + s * 0.4, 0.5 + s * 0.3, 1.0)
                } else if t < 0.4 {
                    let s = (t - 0.2) / 0.2;
                    vec4(0.2 + s * 0.3, 0.7 - s * 0.1, 0.8 - s * 0.6, 1.0)
                } else if t < 0.6 {
                    let s = (t - 0.4) / 0.2;
                    vec4(0.5 + s * 0.4, 0.6 + s * 0.3, 0.2, 1.0)
                } else if t < 0.8 {
                    let s = (t - 0.6) / 0.2;
                    vec4(0.9 - s * 0.3, 0.9 - s * 0.3, 0.2 + s * 0.2, 1.0)
                } else {
                    let s = (t - 0.8) / 0.2;
                    vec4(0.6 + s * 0.4, 0.6 + s * 0.4, 0.4 + s * 0.6, 1.0)
                }
            }
            ColorMap::Rainbow => {
                // HSV-style rainbow
                let h = t * 360.0;
                let sv = 1.0f32;

                let c = sv;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = sv - c;

                let (r, g, b) = if h < 60.0 {
                    (c, x, 0.0f32)
                } else if h < 120.0 {
                    (x, c, 0.0f32)
                } else if h < 180.0 {
                    (0.0f32, c, x)
                } else if h < 240.0 {
                    (0.0f32, x, c)
                } else if h < 300.0 {
                    (x, 0.0f32, c)
                } else {
                    (c, 0.0f32, x)
                };

                vec4(r + m, g + m, b + m, 1.0)
            }
        }
    }
}

/// 3D point
#[derive(Clone, Copy, Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Surface data
#[derive(Clone, Debug, Default)]
pub struct SurfaceData {
    /// Height values in grid (row-major)
    pub heights: Vec<Vec<f64>>,
    pub min_z: f64,
    pub max_z: f64,
}

impl SurfaceData {
    pub fn from_function<F>(resolution: usize, x_range: (f64, f64), y_range: (f64, f64), f: F) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut heights = vec![vec![0.0; resolution]; resolution];
        let mut min_z = f64::MAX;
        let mut max_z = f64::MIN;

        for i in 0..resolution {
            for j in 0..resolution {
                let x = x_range.0 + (x_range.1 - x_range.0) * (i as f64 / (resolution - 1) as f64);
                let y = y_range.0 + (y_range.1 - y_range.0) * (j as f64 / (resolution - 1) as f64);
                let z = f(x, y);
                heights[i][j] = z;
                min_z = min_z.min(z);
                max_z = max_z.max(z);
            }
        }

        Self { heights, min_z, max_z }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SurfacePlotWidget {
    #[redraw]
    #[live]
    draw_line: DrawChartLine,

    #[redraw]
    #[live]
    draw_face: DrawTriangle,

    #[walk]
    walk: Walk,

    #[rust]
    surface_data: SurfaceData,

    #[rust]
    animator: ChartAnimator,

    #[rust]
    initialized: bool,

    #[rust]
    area: Area,

    #[rust]
    chart_rect: Rect,

    #[rust]
    resolution: usize,

    #[rust(0.5)]
    rotation_x: f64,

    #[rust(0.3)]
    rotation_z: f64,

    #[rust]
    color_map: ColorMap,

    #[rust(true)]
    show_wireframe: bool,

    #[rust(true)]
    show_surface: bool,

    #[rust(true)]
    animate_rotation: bool,

    #[rust(0.0)]
    time_offset: f64,
}

impl Widget for SurfacePlotWidget {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event {
            Event::NextFrame(ne) => {
                let time = cx.seconds_since_app_start();

                if self.animator.is_running() {
                    if self.animator.update(time) {
                        self.redraw(cx);
                    }
                }

                // Continuous rotation animation
                if self.animate_rotation && !self.animator.is_running() {
                    self.time_offset = time;
                    self.redraw(cx);
                }

                cx.new_next_frame();
            }
            Event::WindowGeomChange(_) => {
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);

        if rect.size.x > 0.0 && rect.size.y > 0.0 {
            self.chart_rect = rect;

            if !self.initialized {
                self.initialize_data();
                self.start_animation(cx);
                self.initialized = true;
            }

            self.draw_surface(cx);
        }

        DrawStep::done()
    }
}

impl SurfacePlotWidget {
    fn initialize_data(&mut self) {
        // Set default resolution
        if self.resolution == 0 {
            self.resolution = 25;
        }

        // Create interesting mathematical surface
        self.surface_data = SurfaceData::from_function(
            self.resolution,
            (-2.0, 2.0),
            (-2.0, 2.0),
            |x, y| {
                // Combination of sinusoidal functions (like a standing wave)
                let r = (x * x + y * y).sqrt();
                let wave = (r * 3.0).sin() * (-r * 0.3).exp();

                // Add some Gaussian bumps
                let bump1 = (-((x - 0.8).powi(2) + (y - 0.8).powi(2)) / 0.3).exp() * 0.5;
                let bump2 = (-((x + 0.8).powi(2) + (y + 0.8).powi(2)) / 0.4).exp() * 0.4;

                wave + bump1 + bump2
            },
        );

        // Set default color map
        self.color_map = ColorMap::Viridis;
    }

    pub fn set_data(&mut self, data: SurfaceData) {
        self.surface_data = data;
        self.initialized = false;
    }

    pub fn set_color_map(&mut self, color_map: ColorMap) {
        self.color_map = color_map;
    }

    fn start_animation(&mut self, cx: &mut Cx) {
        let time = cx.seconds_since_app_start();
        self.animator = ChartAnimator::new(1500.0)
            .with_easing(EasingType::EaseOutCubic);
        self.animator.start(time);
        self.time_offset = time;
        cx.new_next_frame();
    }

    pub fn replay_animation(&mut self, cx: &mut Cx) {
        self.animator.reset();
        self.start_animation(cx);
        self.redraw(cx);
    }

    fn project_point(&self, p: Point3D, rect: Rect) -> (DVec2, f64) {
        // Get rotation angles (with animation)
        let rot_z = self.rotation_z + if self.animate_rotation {
            self.time_offset * 0.3
        } else {
            0.0
        };

        // Rotate around Z axis
        let cos_z = rot_z.cos();
        let sin_z = rot_z.sin();
        let x1 = p.x * cos_z - p.y * sin_z;
        let y1 = p.x * sin_z + p.y * cos_z;
        let z1 = p.z;

        // Rotate around X axis (tilt)
        let cos_x = self.rotation_x.cos();
        let sin_x = self.rotation_x.sin();
        let y2 = y1 * cos_x - z1 * sin_x;
        let z2 = y1 * sin_x + z1 * cos_x;

        // Isometric-like projection with perspective
        let scale = rect.size.x.min(rect.size.y) * 0.35;
        let perspective = 3.0 / (3.0 + y2 * 0.3);

        let screen_x = rect.pos.x + rect.size.x / 2.0 + x1 * scale * perspective;
        let screen_y = rect.pos.y + rect.size.y / 2.0 - z2 * scale * perspective + y2 * scale * 0.2;

        (dvec2(screen_x, screen_y), y2) // Return depth for sorting
    }

    fn draw_surface(&mut self, cx: &mut Cx2d) {
        let progress = self.animator.get_progress();
        let rect = self.chart_rect;

        let rows = self.surface_data.heights.len();
        if rows < 2 { return; }
        let cols = self.surface_data.heights[0].len();
        if cols < 2 { return; }

        let z_range = self.surface_data.max_z - self.surface_data.min_z;
        if z_range.abs() < 1e-10 { return; }

        // Collect all faces with their average depth for sorting
        struct Face {
            points: [Point3D; 4],
            avg_z: f64,
            color: Vec4,
        }

        let mut faces: Vec<Face> = Vec::new();

        for i in 0..rows - 1 {
            for j in 0..cols - 1 {
                let x0 = (i as f64 / (rows - 1) as f64) * 2.0 - 1.0;
                let x1 = ((i + 1) as f64 / (rows - 1) as f64) * 2.0 - 1.0;
                let y0 = (j as f64 / (cols - 1) as f64) * 2.0 - 1.0;
                let y1 = ((j + 1) as f64 / (cols - 1) as f64) * 2.0 - 1.0;

                let z00 = self.surface_data.heights[i][j] * progress;
                let z01 = self.surface_data.heights[i][j + 1] * progress;
                let z10 = self.surface_data.heights[i + 1][j] * progress;
                let z11 = self.surface_data.heights[i + 1][j + 1] * progress;

                let avg_z = (z00 + z01 + z10 + z11) / 4.0;

                // Normalize for color
                let t = (avg_z / progress.max(0.01) - self.surface_data.min_z) / z_range;
                let color = self.color_map.get_color(t);

                faces.push(Face {
                    points: [
                        Point3D::new(x0, y0, z00),
                        Point3D::new(x1, y0, z10),
                        Point3D::new(x1, y1, z11),
                        Point3D::new(x0, y1, z01),
                    ],
                    avg_z,
                    color,
                });
            }
        }

        // Sort by depth (painter's algorithm - draw far faces first)
        faces.sort_by(|a, b| {
            let (_, da) = self.project_point(Point3D::new(0.0, 0.0, a.avg_z), rect);
            let (_, db) = self.project_point(Point3D::new(0.0, 0.0, b.avg_z), rect);
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Draw faces
        for face in &faces {
            let p0 = self.project_point(face.points[0], rect).0;
            let p1 = self.project_point(face.points[1], rect).0;
            let p2 = self.project_point(face.points[2], rect).0;
            let p3 = self.project_point(face.points[3], rect).0;

            if self.show_surface {
                // Draw two triangles to form quad
                let mut surface_color = face.color;

                // Add slight shading based on normal
                let normal_factor = 0.7 + 0.3 * (face.avg_z / self.surface_data.max_z).abs();
                surface_color.x *= normal_factor as f32;
                surface_color.y *= normal_factor as f32;
                surface_color.z *= normal_factor as f32;

                self.draw_face.color = surface_color;
                self.draw_face.disable_gradient();

                self.draw_face.draw_triangle(cx, p0, p1, p2);
                self.draw_face.draw_triangle(cx, p0, p2, p3);
            }

            if self.show_wireframe {
                // Draw edges
                let line_color = vec4(
                    face.color.x * 0.6,
                    face.color.y * 0.6,
                    face.color.z * 0.6,
                    0.7,
                );
                self.draw_line.color = line_color;

                self.draw_line.draw_line(cx, p0, p1, 1.0);
                self.draw_line.draw_line(cx, p1, p2, 1.0);
                self.draw_line.draw_line(cx, p2, p3, 1.0);
                self.draw_line.draw_line(cx, p3, p0, 1.0);
            }
        }
    }
}
