//! Geographic projections
//!
//! Transforms spherical coordinates (longitude, latitude) to planar coordinates.

use std::f64::consts::PI;

/// Trait for geographic projections
///
/// Projects spherical coordinates (longitude, latitude in degrees) to
/// planar coordinates (x, y in pixels).
pub trait Projection: Send + Sync {
    /// Project geographic coordinates to screen coordinates
    ///
    /// # Arguments
    /// * `lon` - Longitude in degrees (-180 to 180)
    /// * `lat` - Latitude in degrees (-90 to 90)
    ///
    /// # Returns
    /// Screen coordinates (x, y)
    fn project(&self, lon: f64, lat: f64) -> (f64, f64);

    /// Invert screen coordinates back to geographic coordinates
    ///
    /// # Arguments
    /// * `x` - Screen X coordinate
    /// * `y` - Screen Y coordinate
    ///
    /// # Returns
    /// Geographic coordinates (longitude, latitude) in degrees
    fn invert(&self, x: f64, y: f64) -> (f64, f64);

    /// Get the projection type name
    fn projection_type(&self) -> &'static str;

    /// Check if a point is visible in this projection
    fn is_visible(&self, lon: f64, lat: f64) -> bool {
        let _ = (lon, lat);
        true
    }

    /// Get the projection's clip extent
    fn clip_extent(&self) -> Option<((f64, f64), (f64, f64))> {
        None
    }
}

/// Builder trait for projections
pub trait ProjectionBuilder: Sized {
    /// Set the scale factor
    fn scale(self, scale: f64) -> Self;

    /// Set the center point (longitude, latitude)
    fn center(self, lon: f64, lat: f64) -> Self;

    /// Set the translation (screen offset)
    fn translate(self, x: f64, y: f64) -> Self;

    /// Set the rotation
    fn rotate(self, lambda: f64, phi: f64, gamma: f64) -> Self;

    /// Set the clip angle (for azimuthal projections)
    fn clip_angle(self, angle: f64) -> Self;

    /// Set the precision for adaptive resampling
    fn precision(self, precision: f64) -> Self;
}

/// Mercator projection (conformal cylindrical)
///
/// The standard projection for web maps. Preserves angles but distorts
/// areas significantly at high latitudes.
///
/// # Example
///
/// ```
/// use makepad_d3::geo::{MercatorProjection, Projection, ProjectionBuilder};
///
/// let projection = MercatorProjection::new()
///     .scale(100.0)
///     .translate(400.0, 300.0);
///
/// let (x, y) = projection.project(0.0, 0.0); // Null Island
/// assert!((x - 400.0).abs() < 0.01);
/// assert!((y - 300.0).abs() < 0.01);
/// ```
#[derive(Clone, Debug)]
pub struct MercatorProjection {
    /// Scale factor
    scale: f64,
    /// Center longitude
    center_lon: f64,
    /// Center latitude
    center_lat: f64,
    /// Translation X
    translate_x: f64,
    /// Translation Y
    translate_y: f64,
    /// Maximum latitude (clips at ~85.05°)
    max_lat: f64,
}

impl Default for MercatorProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl MercatorProjection {
    /// Create a new Mercator projection
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            center_lon: 0.0,
            center_lat: 0.0,
            translate_x: 0.0,
            translate_y: 0.0,
            max_lat: 85.05113, // atan(sinh(π)) in degrees
        }
    }

    /// Set the maximum latitude
    pub fn max_lat(mut self, lat: f64) -> Self {
        self.max_lat = lat.abs().min(89.99);
        self
    }
}

impl ProjectionBuilder for MercatorProjection {
    fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    fn center(mut self, lon: f64, lat: f64) -> Self {
        self.center_lon = lon;
        self.center_lat = lat;
        self
    }

    fn translate(mut self, x: f64, y: f64) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self
    }

    fn rotate(self, _lambda: f64, _phi: f64, _gamma: f64) -> Self {
        // Mercator doesn't support rotation in the traditional sense
        self
    }

    fn clip_angle(self, _angle: f64) -> Self {
        self
    }

    fn precision(self, _precision: f64) -> Self {
        self
    }
}

impl Projection for MercatorProjection {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64) {
        // Clamp latitude to avoid infinity
        let lat = lat.clamp(-self.max_lat, self.max_lat);

        // Convert to radians
        let lambda = (lon - self.center_lon).to_radians();
        let phi = lat.to_radians();

        // Mercator projection formula
        let x = lambda;
        let y = (PI / 4.0 + phi / 2.0).tan().ln();

        // Apply scale and translate
        (
            x * self.scale + self.translate_x,
            -y * self.scale + self.translate_y, // Y is inverted
        )
    }

    fn invert(&self, x: f64, y: f64) -> (f64, f64) {
        // Remove scale and translate
        let px = (x - self.translate_x) / self.scale;
        let py = -(y - self.translate_y) / self.scale;

        // Inverse Mercator
        let lon = px.to_degrees() + self.center_lon;
        let lat = (2.0 * py.exp().atan() - PI / 2.0).to_degrees();

        (lon, lat)
    }

    fn projection_type(&self) -> &'static str {
        "mercator"
    }
}

/// Equirectangular projection (plate carrée)
///
/// The simplest projection - directly maps longitude to x and latitude to y.
/// Good for data visualization but has significant distortion at high latitudes.
///
/// # Example
///
/// ```
/// use makepad_d3::geo::{EquirectangularProjection, Projection, ProjectionBuilder};
///
/// let projection = EquirectangularProjection::new()
///     .scale(100.0)
///     .translate(400.0, 300.0);
///
/// // Project the center point (0, 0) - should map to translate coordinates
/// let (x, y) = projection.project(0.0, 0.0);
/// assert!((x - 400.0).abs() < 0.01);
/// assert!((y - 300.0).abs() < 0.01);
/// ```
#[derive(Clone, Debug)]
pub struct EquirectangularProjection {
    /// Scale factor
    scale: f64,
    /// Center longitude
    center_lon: f64,
    /// Center latitude
    center_lat: f64,
    /// Translation X
    translate_x: f64,
    /// Translation Y
    translate_y: f64,
}

impl Default for EquirectangularProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl EquirectangularProjection {
    /// Create a new equirectangular projection
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            center_lon: 0.0,
            center_lat: 0.0,
            translate_x: 0.0,
            translate_y: 0.0,
        }
    }
}

impl ProjectionBuilder for EquirectangularProjection {
    fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    fn center(mut self, lon: f64, lat: f64) -> Self {
        self.center_lon = lon;
        self.center_lat = lat;
        self
    }

    fn translate(mut self, x: f64, y: f64) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self
    }

    fn rotate(self, _lambda: f64, _phi: f64, _gamma: f64) -> Self {
        self
    }

    fn clip_angle(self, _angle: f64) -> Self {
        self
    }

    fn precision(self, _precision: f64) -> Self {
        self
    }
}

impl Projection for EquirectangularProjection {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64) {
        let x = (lon - self.center_lon).to_radians();
        let y = (lat - self.center_lat).to_radians();

        (
            x * self.scale + self.translate_x,
            -y * self.scale + self.translate_y,
        )
    }

    fn invert(&self, x: f64, y: f64) -> (f64, f64) {
        let px = (x - self.translate_x) / self.scale;
        let py = -(y - self.translate_y) / self.scale;

        let lon = px.to_degrees() + self.center_lon;
        let lat = py.to_degrees() + self.center_lat;

        (lon, lat)
    }

    fn projection_type(&self) -> &'static str {
        "equirectangular"
    }
}

/// Orthographic projection (azimuthal)
///
/// Shows the Earth as a globe viewed from space. Points on the far side
/// of the globe are not visible.
///
/// # Example
///
/// ```
/// use makepad_d3::geo::{OrthographicProjection, Projection, ProjectionBuilder};
///
/// let projection = OrthographicProjection::new()
///     .scale(200.0)
///     .translate(400.0, 300.0);
///
/// // Project the center (0, 0) - should map to translate coordinates
/// let (x, y) = projection.project(0.0, 0.0);
/// assert!((x - 400.0).abs() < 0.01);
/// assert!((y - 300.0).abs() < 0.01);
/// ```
#[derive(Clone, Debug)]
pub struct OrthographicProjection {
    /// Scale factor
    scale: f64,
    /// Translation X
    translate_x: f64,
    /// Translation Y
    translate_y: f64,
    /// Rotation: lambda (longitude)
    rotate_lambda: f64,
    /// Rotation: phi (latitude)
    rotate_phi: f64,
    /// Rotation: gamma (roll)
    rotate_gamma: f64,
    /// Clip angle in degrees (default 90°)
    clip_angle: f64,
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl OrthographicProjection {
    /// Create a new orthographic projection
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
            rotate_lambda: 0.0,
            rotate_phi: 0.0,
            rotate_gamma: 0.0,
            clip_angle: 90.0,
        }
    }

    /// Apply rotation to convert to rotated coordinates
    fn rotate_point(&self, lon: f64, lat: f64) -> (f64, f64) {
        let lambda = lon.to_radians() + self.rotate_lambda.to_radians();
        let phi = lat.to_radians();

        // Apply rotation around the pole
        let cos_phi = phi.cos();
        let sin_phi = phi.sin();
        let cos_gamma = self.rotate_phi.to_radians().cos();
        let sin_gamma = self.rotate_phi.to_radians().sin();

        let x = cos_phi * lambda.cos();
        let y = cos_phi * lambda.sin();
        let z = sin_phi;

        // Rotate around Y axis (phi rotation)
        let x2 = x * cos_gamma + z * sin_gamma;
        let y2 = y;
        let z2 = -x * sin_gamma + z * cos_gamma;

        // Convert back to spherical
        let new_lon = y2.atan2(x2);
        let new_lat = z2.asin();

        (new_lon, new_lat)
    }
}

impl ProjectionBuilder for OrthographicProjection {
    fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    fn center(self, _lon: f64, _lat: f64) -> Self {
        // For orthographic, use rotate instead
        self
    }

    fn translate(mut self, x: f64, y: f64) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self
    }

    fn rotate(mut self, lambda: f64, phi: f64, gamma: f64) -> Self {
        self.rotate_lambda = lambda;
        self.rotate_phi = phi;
        self.rotate_gamma = gamma;
        self
    }

    fn clip_angle(mut self, angle: f64) -> Self {
        self.clip_angle = angle.clamp(0.0, 180.0);
        self
    }

    fn precision(self, _precision: f64) -> Self {
        self
    }
}

impl Projection for OrthographicProjection {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64) {
        let (lambda, phi) = self.rotate_point(lon, lat);

        // Orthographic projection
        let x = phi.cos() * lambda.sin();
        let y = phi.sin();

        (
            x * self.scale + self.translate_x,
            -y * self.scale + self.translate_y,
        )
    }

    fn invert(&self, x: f64, y: f64) -> (f64, f64) {
        let px = (x - self.translate_x) / self.scale;
        let py = -(y - self.translate_y) / self.scale;

        let rho = (px * px + py * py).sqrt();
        if rho > 1.0 {
            // Point is outside the globe
            return (f64::NAN, f64::NAN);
        }

        let c = rho.asin();
        let sin_c = c.sin();
        let cos_c = c.cos();

        let lat = if rho == 0.0 {
            0.0
        } else {
            (py * sin_c / rho).asin()
        };

        let lon = (px * sin_c).atan2(rho * cos_c);

        // Reverse rotation (simplified - full inverse rotation would be more complex)
        let final_lon = lon.to_degrees() - self.rotate_lambda;
        let final_lat = lat.to_degrees();

        (final_lon, final_lat)
    }

    fn projection_type(&self) -> &'static str {
        "orthographic"
    }

    fn is_visible(&self, lon: f64, lat: f64) -> bool {
        let (lambda, phi) = self.rotate_point(lon, lat);
        let cos_c = phi.cos() * lambda.cos();
        cos_c >= self.clip_angle.to_radians().cos()
    }
}

/// Albers equal-area conic projection
///
/// Good for maps of countries/regions at mid-latitudes (like the US).
/// Preserves area but not shape.
///
/// # Example
///
/// ```
/// use makepad_d3::geo::{AlbersProjection, Projection, ProjectionBuilder};
///
/// let projection = AlbersProjection::usa()
///     .scale(1000.0)
///     .translate(480.0, 300.0);
///
/// // Project a point in the continental US
/// let (x, y) = projection.project(-98.0, 39.0);
/// ```
#[derive(Clone, Debug)]
pub struct AlbersProjection {
    /// Scale factor
    scale: f64,
    /// Translation X
    translate_x: f64,
    /// Translation Y
    translate_y: f64,
    /// Center longitude
    center_lon: f64,
    /// Center latitude
    center_lat: f64,
    /// First standard parallel
    parallel1: f64,
    /// Second standard parallel
    parallel2: f64,
    // Precomputed values
    n: f64,
    c: f64,
    rho0: f64,
}

impl Default for AlbersProjection {
    fn default() -> Self {
        Self::new()
    }
}

impl AlbersProjection {
    /// Create a new Albers projection with default parameters
    pub fn new() -> Self {
        Self::with_parallels(29.5, 45.5)
    }

    /// Create an Albers projection optimized for the USA
    pub fn usa() -> Self {
        Self::with_parallels(29.5, 45.5)
            .center(-98.0, 39.0)
    }

    /// Create with custom standard parallels
    pub fn with_parallels(parallel1: f64, parallel2: f64) -> Self {
        let mut proj = Self {
            scale: 1.0,
            translate_x: 0.0,
            translate_y: 0.0,
            center_lon: 0.0,
            center_lat: 0.0,
            parallel1,
            parallel2,
            n: 0.0,
            c: 0.0,
            rho0: 0.0,
        };
        proj.compute_constants();
        proj
    }

    /// Set standard parallels
    pub fn parallels(mut self, p1: f64, p2: f64) -> Self {
        self.parallel1 = p1;
        self.parallel2 = p2;
        self.compute_constants();
        self
    }

    fn compute_constants(&mut self) {
        let phi1 = self.parallel1.to_radians();
        let phi2 = self.parallel2.to_radians();
        let phi0 = self.center_lat.to_radians();

        let sin_phi1 = phi1.sin();
        let sin_phi2 = phi2.sin();
        let cos_phi1 = phi1.cos();

        self.n = (sin_phi1 + sin_phi2) / 2.0;
        self.c = cos_phi1 * cos_phi1 + 2.0 * self.n * sin_phi1;
        self.rho0 = (self.c - 2.0 * self.n * phi0.sin()).sqrt() / self.n;
    }
}

impl ProjectionBuilder for AlbersProjection {
    fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    fn center(mut self, lon: f64, lat: f64) -> Self {
        self.center_lon = lon;
        self.center_lat = lat;
        self.compute_constants();
        self
    }

    fn translate(mut self, x: f64, y: f64) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self
    }

    fn rotate(self, _lambda: f64, _phi: f64, _gamma: f64) -> Self {
        self
    }

    fn clip_angle(self, _angle: f64) -> Self {
        self
    }

    fn precision(self, _precision: f64) -> Self {
        self
    }
}

impl Projection for AlbersProjection {
    fn project(&self, lon: f64, lat: f64) -> (f64, f64) {
        let lambda = (lon - self.center_lon).to_radians();
        let phi = lat.to_radians();

        let rho = (self.c - 2.0 * self.n * phi.sin()).sqrt() / self.n;
        let theta = self.n * lambda;

        let x = rho * theta.sin();
        let y = self.rho0 - rho * theta.cos();

        (
            x * self.scale + self.translate_x,
            y * self.scale + self.translate_y,
        )
    }

    fn invert(&self, x: f64, y: f64) -> (f64, f64) {
        let px = (x - self.translate_x) / self.scale;
        let py = (y - self.translate_y) / self.scale;

        let rho0_minus_y = self.rho0 - py;
        let rho = (px * px + rho0_minus_y * rho0_minus_y).sqrt();
        let rho = if self.n < 0.0 { -rho } else { rho };

        let theta = px.atan2(rho0_minus_y);

        let lon = (theta / self.n).to_degrees() + self.center_lon;
        let lat = ((self.c - rho * rho * self.n * self.n) / (2.0 * self.n)).asin().to_degrees();

        (lon, lat)
    }

    fn projection_type(&self) -> &'static str {
        "albers"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mercator_new() {
        let proj = MercatorProjection::new();
        assert_eq!(proj.projection_type(), "mercator");
    }

    #[test]
    fn test_mercator_project_origin() {
        let proj = MercatorProjection::new()
            .scale(100.0)
            .translate(400.0, 300.0);

        let (x, y) = proj.project(0.0, 0.0);
        assert!((x - 400.0).abs() < 0.01);
        assert!((y - 300.0).abs() < 0.01);
    }

    #[test]
    fn test_mercator_roundtrip() {
        let proj = MercatorProjection::new()
            .scale(100.0)
            .translate(400.0, 300.0);

        let original = (-122.4, 37.8); // San Francisco
        let (x, y) = proj.project(original.0, original.1);
        let (lon, lat) = proj.invert(x, y);

        assert!((lon - original.0).abs() < 0.01);
        assert!((lat - original.1).abs() < 0.01);
    }

    #[test]
    fn test_equirectangular_new() {
        let proj = EquirectangularProjection::new();
        assert_eq!(proj.projection_type(), "equirectangular");
    }

    #[test]
    fn test_equirectangular_project() {
        let proj = EquirectangularProjection::new()
            .scale(1.0)
            .translate(0.0, 0.0);

        let (x, y) = proj.project(90.0, 0.0);
        assert!((x - PI / 2.0).abs() < 0.01);
        assert!(y.abs() < 0.01);
    }

    #[test]
    fn test_equirectangular_roundtrip() {
        let proj = EquirectangularProjection::new()
            .scale(100.0)
            .translate(200.0, 100.0);

        let original = (45.0, 30.0);
        let (x, y) = proj.project(original.0, original.1);
        let (lon, lat) = proj.invert(x, y);

        assert!((lon - original.0).abs() < 0.01);
        assert!((lat - original.1).abs() < 0.01);
    }

    #[test]
    fn test_orthographic_new() {
        let proj = OrthographicProjection::new();
        assert_eq!(proj.projection_type(), "orthographic");
    }

    #[test]
    fn test_orthographic_center() {
        let proj = OrthographicProjection::new()
            .scale(200.0)
            .translate(400.0, 300.0)
            .rotate(0.0, 0.0, 0.0);

        // Point at center should project to translate point
        let (x, y) = proj.project(0.0, 0.0);
        assert!((x - 400.0).abs() < 0.01);
        assert!((y - 300.0).abs() < 0.01);
    }

    #[test]
    fn test_orthographic_visibility() {
        let proj = OrthographicProjection::new()
            .rotate(0.0, 0.0, 0.0);

        // Front of globe should be visible
        assert!(proj.is_visible(0.0, 0.0));

        // Back of globe should not be visible
        assert!(!proj.is_visible(180.0, 0.0));
    }

    #[test]
    fn test_albers_new() {
        let proj = AlbersProjection::new();
        assert_eq!(proj.projection_type(), "albers");
    }

    #[test]
    fn test_albers_usa() {
        let proj = AlbersProjection::usa()
            .scale(1000.0)
            .translate(480.0, 300.0);

        // Center point should project near translate
        let (x, y) = proj.project(-98.0, 39.0);
        assert!((x - 480.0).abs() < 50.0);
        assert!((y - 300.0).abs() < 50.0);
    }

    #[test]
    fn test_albers_roundtrip() {
        let proj = AlbersProjection::usa()
            .scale(1000.0)
            .translate(480.0, 300.0);

        let original = (-100.0, 40.0);
        let (x, y) = proj.project(original.0, original.1);
        let (lon, lat) = proj.invert(x, y);

        assert!((lon - original.0).abs() < 0.1);
        assert!((lat - original.1).abs() < 0.1);
    }
}
