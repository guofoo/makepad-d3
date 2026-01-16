//! Scale implementations for data visualization
//!
//! Scales are functions that map from an input domain to an output range.
//! This module provides various scale types:
//!
//! - [`LinearScale`]: Linear interpolation between domain and range
//! - [`CategoryScale`]: Maps discrete categories to continuous bands
//! - [`BandScale`]: Maps discrete categories to bands with configurable padding (D3-compatible)
//! - [`PointScale`]: Maps discrete categories to evenly spaced points (zero bandwidth)
//! - [`QuantizeScale`]: Maps continuous domain to discrete range (equal-sized segments)
//! - [`QuantileScale`]: Maps continuous domain to discrete range (equal-count segments based on data)
//! - [`ThresholdScale`]: Maps continuous domain to discrete range (custom breakpoints)
//! - [`SequentialScale`]: Maps continuous domain through an interpolator (for color gradients)
//! - [`TimeScale`]: Maps DateTime values to continuous range
//! - [`LogScale`]: Logarithmic interpolation for exponential data
//! - [`PowScale`]: Power/polynomial interpolation
//! - [`SymlogScale`]: Symmetric log for data crossing zero
//!
//! # Example
//! ```
//! use makepad_d3::scale::{Scale, LinearScale, ScaleExt};
//!
//! let scale = LinearScale::new()
//!     .with_domain(0.0, 100.0)
//!     .with_range(0.0, 500.0);
//!
//! assert_eq!(scale.scale(50.0), 250.0);
//! ```

mod traits;
mod utils;
mod linear;
mod category;
mod band;
mod point;
mod quantize;
mod quantile;
mod threshold;
mod sequential;
mod time;
mod log;
mod pow;
mod symlog;

pub use traits::{Scale, ContinuousScale, DiscreteScale, ScaleExt, Tick, TickOptions};
pub use utils::{nice_step, nice_bounds, format_number};
pub use linear::LinearScale;
pub use category::CategoryScale;
pub use band::BandScale;
pub use point::PointScale;
pub use quantize::QuantizeScale;
pub use quantile::QuantileScale;
pub use threshold::ThresholdScale;
pub use sequential::{SequentialScale, interpolators};
pub use time::{TimeScale, TimeTick, TimeInterval};
pub use log::LogScale;
pub use pow::PowScale;
pub use symlog::SymlogScale;
