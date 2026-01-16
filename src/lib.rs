//! Makepad D3 - D3.js-compatible data visualization library for Makepad
//!
//! This library provides data visualization primitives inspired by D3.js,
//! optimized for Makepad's GPU-accelerated rendering.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use makepad_d3::prelude::*;
//!
//! // Create chart data
//! let data = ChartData::new()
//!     .with_labels(vec!["Jan", "Feb", "Mar", "Apr"])
//!     .add_dataset(
//!         Dataset::new("Revenue")
//!             .with_data(vec![100.0, 200.0, 150.0, 300.0])
//!             .with_hex_color(0x4285F4)
//!     );
//!
//! // Create scales
//! let x_scale = CategoryScale::new()
//!     .with_labels(data.labels.clone())
//!     .with_range(50.0, 550.0);
//!
//! let y_scale = LinearScale::new()
//!     .with_domain(0.0, 300.0)
//!     .with_range(350.0, 50.0);  // Inverted for screen coordinates
//! ```
//!
//! # Modules
//!
//! - [`data`]: Data structures for charts (DataPoint, Dataset, ChartData)
//! - [`scale`]: Scale functions for mapping data to visual space
//! - [`axis`]: Axis components for tick marks, labels, and formatting
//! - [`shape`]: Shape generators (lines, areas, arcs, pies, stacks)
//! - [`color`]: Color scales and schemes (sequential, diverging, categorical)
//! - [`interaction`]: Interactive behaviors (zoom, brush, tooltip)
//! - [`layout`]: Layout algorithms (force simulation, tree, treemap, pack)
//! - [`geo`]: Geographic projections and GeoJSON support
//! - [`component`]: Reusable UI components (legend, tooltip, crosshair, annotation)
//! - [`error`]: Error types
//!
//! # Features
//!
//! - **Scales**: Linear, Category, Time, Log, Pow, Symlog
//! - **Data Structures**: Flexible data containers with builder patterns
//! - **Serialization**: Full serde support for data import/export

#![warn(missing_docs)]
#![warn(clippy::all)]

pub mod error;
pub mod data;
pub mod scale;
pub mod axis;
pub mod shape;
pub mod color;
pub mod interaction;
pub mod layout;
pub mod geo;
pub mod component;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::error::{D3Error, D3Result};
    pub use crate::data::{DataPoint, Dataset, PointStyle, ChartData, Color};
    pub use crate::scale::{
        Scale, ContinuousScale, DiscreteScale, ScaleExt,
        LinearScale, CategoryScale,
        TimeScale, TimeTick, TimeInterval,
        LogScale, PowScale, SymlogScale,
        Tick, TickOptions,
        nice_step, nice_bounds, format_number,
    };
    pub use crate::axis::{
        Axis, AxisConfig, AxisLayout, AxisOrientation, AxisTick,
        NumberFormat, DurationFormat, format_si,
    };
    pub use crate::shape::{
        Path, PathSegment, Point,
        LineGenerator, AreaGenerator,
        ArcGenerator, ArcDatum,
        PieLayout, PieSlice, PieSort,
        StackGenerator, StackedSeries, StackPoint, StackOrder, StackOffset,
    };
    pub use crate::color::{
        Rgba, Hsl,
        ColorScale, SequentialScale, DivergingScale, CategoricalScale,
        lerp_color, hex, rgb, rgba, hsl,
    };
    pub use crate::interaction::{
        ZoomTransform, ZoomBehavior,
        BrushType, BrushBehavior, BrushSelection,
        TooltipContent,
    };
    pub use crate::layout::{
        ForceSimulation, SimulationNode, SimulationLink,
        Force, ManyBodyForce, LinkForce, CollideForce, CenterForce, PositionForce, RadialForce,
        HierarchyNode, TreeLayout, TreemapLayout, PackLayout,
        TilingMethod, PackStrategy,
    };
    pub use crate::geo::{
        Projection, ProjectionBuilder,
        MercatorProjection, EquirectangularProjection, OrthographicProjection, AlbersProjection,
        GeoJson, Feature, FeatureCollection, Geometry, GeometryType,
        Position, BoundingBox, Properties,
        GeoPath, GeoPathSegment,
    };
    pub use crate::component::{
        Legend, LegendItem, LegendOrientation, LegendPosition,
        TooltipWidget, TooltipConfig,
        Crosshair, CrosshairMode,
        Annotation, AnnotationLayer, AnnotationType,
        ReferenceLine, ReferenceLineSet,
    };
}

// Re-export Color from data module at crate root for convenience
pub use data::Color;
