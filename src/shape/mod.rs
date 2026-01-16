//! Shape generators for data visualization
//!
//! This module provides D3-style shape generators for creating drawable paths
//! from data. Shapes include lines, areas, arcs, and pie layouts.
//!
//! # Modules
//!
//! - [`curve`]: Curve interpolation algorithms (linear, step, cardinal, etc.)
//! - [`path`]: Path segment primitives
//!
//! # Generators
//!
//! - [`LineGenerator`]: Generate line paths from data points
//! - [`AreaGenerator`]: Generate filled area paths
//! - [`ArcGenerator`]: Generate arc paths for pie/donut charts
//! - [`PieLayout`]: Compute pie slice angles from values
//! - [`StackGenerator`]: Compute stacked layouts for bar/area charts
//!
//! # Example
//!
//! ```
//! use makepad_d3::data::DataPoint;
//! use makepad_d3::shape::{LineGenerator, Point};
//! use makepad_d3::shape::curve::MonotoneCurve;
//!
//! let data = vec![
//!     DataPoint::from((0.0, 100.0)),
//!     DataPoint::from((50.0, 150.0)),
//!     DataPoint::from((100.0, 120.0)),
//! ];
//!
//! let line = LineGenerator::new().curve(MonotoneCurve::new());
//! let path = line.generate(&data);
//! ```

pub mod curve;
pub mod path;

mod line;
mod area;
mod arc;
mod pie;
mod stack;

pub use path::{Path, PathSegment, Point};
pub use line::LineGenerator;
pub use area::AreaGenerator;
pub use arc::{ArcGenerator, ArcDatum};
pub use pie::{PieLayout, PieSlice, PieSort};
pub use stack::{StackGenerator, StackedSeries, StackPoint, StackOrder, StackOffset};
