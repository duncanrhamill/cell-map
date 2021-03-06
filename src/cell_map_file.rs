//! Provides the [`CellMapFile`] type which allows a cell map to be serialised using serde.

// ------------------------------------------------------------------------------------------------
// IMPORTS
// ------------------------------------------------------------------------------------------------

use std::convert::TryFrom;

use nalgebra::{Affine2, Vector2};
use ndarray::Array2;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{cell_map::Bounds, CellMap, CellMapParams, Error, Layer};

// ------------------------------------------------------------------------------------------------
// STRUCTS
// ------------------------------------------------------------------------------------------------

/// Represents a file that can be serialised and deserialised using serde.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellMapFile<L, T>
where
    L: Layer,
{
    /// Number of layers stored in the map
    pub num_layers: usize,

    /// The order of layers in the map.
    ///
    /// The index of a layer name in this vector matches the index of that layer in the `data`
    /// member.
    pub layers: Vec<L>,

    /// The bounds of the map
    pub cell_bounds: Bounds,

    /// The size of each cell in the map, in parent-frame units.
    pub cell_size: Vector2<f64>,

    /// The precision used when calculating cell boundaries, relative to `cell_size`.
    pub cell_boundary_precision: f64,

    /// The angle which rotates the parent frame into the map frame, in radians.
    pub from_parent_angle_rad: f64,

    /// The translation that goes from the parent frame to the map frame, in parent frame units.
    pub from_parent_translation: Vector2<f64>,

    /// The affine transformation matrix that converts from points in the parent frame to the map frame.
    pub from_parent_matrix: Affine2<f64>,

    /// Stores each layer of the map as an [`ndarray::Array2<T>`].
    pub data: Vec<Array2<T>>,
}

// ------------------------------------------------------------------------------------------------
// IMPLS
// ------------------------------------------------------------------------------------------------

impl<L, T> CellMapFile<L, T>
where
    L: Layer,
{
    /// Converts this file into a [`CellMap`].
    pub fn into_cell_map(self) -> Result<CellMap<L, T>, Error> {
        let params = CellMapParams {
            cell_size: self.cell_size,
            cell_bounds: self.cell_bounds,
            rotation_in_parent_rad: self.from_parent_angle_rad,
            position_in_parent: self.from_parent_translation,
            cell_boundary_precision: self.cell_boundary_precision,
        };

        CellMap::new_from_data(params, self.data)
    }
}

impl<L, T> CellMapFile<L, T>
where
    L: Layer + Serialize,
    T: Clone + Serialize,
{
    pub(crate) fn new(map: &CellMap<L, T>) -> Self {
        Self {
            num_layers: L::NUM_LAYERS,
            layers: L::all(),
            cell_bounds: map.metadata.cell_bounds,
            cell_size: map.metadata.cell_size,
            cell_boundary_precision: map.metadata.cell_boundary_precision,
            from_parent_angle_rad: map.params.rotation_in_parent_rad,
            from_parent_translation: map.params.position_in_parent,
            from_parent_matrix: map.metadata.to_parent.inverse(),
            data: map.data.clone(),
        }
    }
}

impl<L, T> CellMapFile<L, T>
where
    L: Layer + Serialize,
    T: Serialize,
{
    /// Writes the [`CellMapFile`] to the given path, overwriting any existing file. The format of
    /// the written file is JSON.
    #[cfg(feature = "json")]
    pub fn write_json<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Error> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(false)
            .truncate(true)
            .write(true)
            .open(path)
            .map_err(Error::IoError)?;

        serde_json::to_writer_pretty(file, &self).map_err(Error::JsonError)?;

        Ok(())
    }
}

impl<L, T> CellMapFile<L, T>
where
    L: Layer + DeserializeOwned,
    T: DeserializeOwned,
{
    /// Loads a [`CellMapFile`] from the given path, which points to a JSON file.
    #[cfg(feature = "json")]
    pub fn from_json<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Error> {
        // Open the file
        let file = std::fs::File::open(path).map_err(Error::IoError)?;
        let map_file: CellMapFile<L, T> =
            serde_json::from_reader(&file).map_err(Error::JsonError)?;
        Ok(map_file)
    }
}

impl<L, T> From<CellMap<L, T>> for CellMapFile<L, T>
where
    L: Layer + Serialize,
    T: Clone + Serialize,
{
    fn from(map: CellMap<L, T>) -> Self {
        Self::new(&map)
    }
}

impl<L, T> TryFrom<CellMapFile<L, T>> for CellMap<L, T>
where
    L: Layer,
{
    type Error = Error;

    fn try_from(value: CellMapFile<L, T>) -> Result<Self, Self::Error> {
        value.into_cell_map()
    }
}
