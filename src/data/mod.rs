//! Data structures and sources for chart data
//!
//! This module provides:
//! - Core data structures ([`DataPoint`], [`Dataset`], [`ChartData`])
//! - Dynamic data sources ([`DataSource`], [`BufferedDataSource`], [`StreamingDataSource`])
//! - Observable datasets with change tracking ([`ObservableDataset`])
//! - Data transformation pipelines ([`DataPipeline`])
//!
//! # Static Data Example
//!
//! ```
//! use makepad_d3::data::{ChartData, Dataset};
//!
//! let data = ChartData::new()
//!     .with_labels(vec!["Jan", "Feb", "Mar"])
//!     .add_dataset(Dataset::new("Revenue").with_data(vec![100.0, 200.0, 150.0]));
//! ```
//!
//! # Dynamic Data Example
//!
//! ```
//! use makepad_d3::data::{ObservableDataset, DataPoint};
//!
//! let mut dataset = ObservableDataset::new("Live Data");
//! dataset.push(DataPoint::from_y(100.0));
//!
//! // Check for changes
//! while let Some(change) = dataset.poll_change() {
//!     println!("Data changed: {:?}", change);
//! }
//! ```
//!
//! # Streaming Data Example
//!
//! ```
//! use makepad_d3::data::{StreamingDataSource, StreamMessage, DataPoint};
//!
//! let (mut source, tx) = StreamingDataSource::new();
//!
//! // Send data from another thread
//! tx.send(StreamMessage::Point(DataPoint::from_y(42.0))).unwrap();
//!
//! // Poll in render loop
//! let event = source.poll();
//! ```

mod point;
mod dataset;
mod chart_data;
mod source;
mod observable;
mod streaming;
mod polling;
mod pipeline;

// Core data structures
pub use point::DataPoint;
pub use dataset::{Dataset, PointStyle, Color};
pub use chart_data::ChartData;

// Data source traits and types
pub use source::{
    DataSource,
    DataSourceEvent,
    DataSourceState,
    DataSourceConfig,
    BufferedDataSource,
    MultiSeriesDataSource,
};

// Observable dataset
pub use observable::{
    ObservableDataset,
    DataChange,
};

// Streaming data source
pub use streaming::{
    StreamingDataSource,
    StreamMessage,
    SharedStreamingSource,
    StreamingSourceBuilder,
};

// Polling data source
pub use polling::{
    PollingDataSource,
    PollingConfig,
    PollingStrategy,
    PollingState,
    PollingSourceBuilder,
};

// Data pipeline
pub use pipeline::{
    DataPipeline,
    Transform,
    Aggregation,
};
