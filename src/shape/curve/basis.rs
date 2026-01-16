//! Basis spline (B-spline) curve interpolation

use super::{Curve, PathSegment, Point};

/// B-spline curve
///
/// Creates a smooth curve using B-spline interpolation. The curve does not
/// pass through the control points but creates a smooth approximation.
///
/// # Example
/// ```
/// use makepad_d3::shape::curve::{Curve, BasisCurve};
/// use makepad_d3::shape::Point;
///
/// let curve = BasisCurve::new();
/// let points = vec![
///     Point::new(0.0, 0.0),
///     Point::new(50.0, 100.0),
///     Point::new(100.0, 50.0),
///     Point::new(150.0, 100.0),
/// ];
/// let path = curve.generate(&points);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub struct BasisCurve;

impl BasisCurve {
    /// Create a new basis curve
    pub fn new() -> Self {
        Self
    }
}

impl Curve for BasisCurve {
    fn generate(&self, points: &[Point]) -> Vec<PathSegment> {
        if points.is_empty() {
            return vec![];
        }

        if points.len() == 1 {
            return vec![PathSegment::MoveTo(points[0])];
        }

        if points.len() == 2 {
            return vec![
                PathSegment::MoveTo(points[0]),
                PathSegment::LineTo(points[1]),
            ];
        }

        let mut path = Vec::new();

        // B-spline basis functions for uniform cubic B-spline
        // The curve starts at a blend of the first few points
        let p0 = points[0];
        let p1 = points[1];

        // Start point: weighted average
        let start = Point::new(
            (p0.x + 4.0 * p0.x + p1.x) / 6.0,
            (p0.y + 4.0 * p0.y + p1.y) / 6.0,
        );
        path.push(PathSegment::MoveTo(start));

        // Generate curve segments
        for i in 1..points.len() - 1 {
            let p0 = points[i - 1];
            let p1 = points[i];
            let p2 = points[i + 1];

            // B-spline control points for this segment
            let cp1 = Point::new(
                (2.0 * p0.x + p1.x) / 3.0,
                (2.0 * p0.y + p1.y) / 3.0,
            );
            let cp2 = Point::new(
                (p0.x + 2.0 * p1.x) / 3.0,
                (p0.y + 2.0 * p1.y) / 3.0,
            );
            let end = Point::new(
                (p0.x + 4.0 * p1.x + p2.x) / 6.0,
                (p0.y + 4.0 * p1.y + p2.y) / 6.0,
            );

            path.push(PathSegment::CurveTo { cp1, cp2, end });
        }

        // End segment
        let n = points.len();
        let p0 = points[n - 2];
        let p1 = points[n - 1];

        let cp1 = Point::new(
            (2.0 * p0.x + p1.x) / 3.0,
            (2.0 * p0.y + p1.y) / 3.0,
        );
        let cp2 = Point::new(
            (p0.x + 2.0 * p1.x) / 3.0,
            (p0.y + 2.0 * p1.y) / 3.0,
        );
        let end = Point::new(
            (p0.x + 4.0 * p1.x + p1.x) / 6.0,
            (p0.y + 4.0 * p1.y + p1.y) / 6.0,
        );

        path.push(PathSegment::CurveTo { cp1, cp2, end });

        path
    }

    fn curve_type(&self) -> &'static str {
        "basis"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basis_basic() {
        let curve = BasisCurve::new();
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(50.0, 100.0),
            Point::new(100.0, 50.0),
            Point::new(150.0, 100.0),
        ];

        let path = curve.generate(&points);
        assert!(!path.is_empty());

        // First segment should be MoveTo
        match &path[0] {
            PathSegment::MoveTo(_) => {}
            _ => panic!("Expected MoveTo"),
        }
    }

    #[test]
    fn test_basis_two_points() {
        let curve = BasisCurve::new();
        let points = vec![
            Point::new(0.0, 0.0),
            Point::new(100.0, 100.0),
        ];

        let path = curve.generate(&points);
        assert_eq!(path.len(), 2); // Falls back to linear
    }
}
