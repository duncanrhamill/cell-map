//! Defines the standard error type for errors related to [`CellMap`]s.
//!
//! [`CellMap`]: crate::CellMap

use nalgebra::{Point2, Vector2};

use crate::cell_map::Bounds;

// ------------------------------------------------------------------------------------------------
// ENUMS
// ------------------------------------------------------------------------------------------------

/// Standard error type for errors related to [`CellMap`]s.
///
/// [`CellMap`]: crate::CellMap
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error returned when trying to construct a [`Windows`] slicer using a `semi_width` which
    /// would create a window larger than the size of the map.
    ///
    /// [`Windows`]: crate::iterators::slicers::Windows
    #[error("Can't create a Windows iterator since the window size ({0}) is larger than the map size ({1})")]
    WindowLargerThanMap(Vector2<usize>, Vector2<usize>),

    /// The given parent-frame position (name, first element) is outside the map.
    #[error("Parent-frame position {0} ({1}) is outside the map")]
    PositionOutsideMap(String, Point2<f64>),

    /// The given index is outside the map.
    #[error("The index {0} is outside the map")]
    IndexOutsideMap(Point2<usize>),

    /// Wrong number of layers, got (first) but expected (second)
    #[error("Expected {0} layers but found {1}")]
    WrongNumberOfLayers(usize, usize),

    /// Wrong shape of layer, got (first) but expected (second)
    #[error("Expected {0:?} cells in layer, but found {1:?}")]
    LayerWrongShape((usize, usize), (usize, usize)),

    /// Errors associated with `std::io` operations.
    #[error("An IO error occured: {0}")]
    IoError(std::io::Error),

    /// Errors associated with `serde_json` operations.
    #[cfg(feature = "json")]
    #[error("Error in serde_json: {0}")]
    JsonError(serde_json::Error),

    /// Error when bounds are invalid, i.e. the minimum is larger than the maximum
    #[error("The provided bounds are not valid: {0:?}")]
    InvalidBounds(Bounds),
}
