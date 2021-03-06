//! Tests for iterators

// ------------------------------------------------------------------------------------------------
// IMPORTS
// ------------------------------------------------------------------------------------------------

use super::*;
use crate::{cell_map::Bounds, test_utils::TestLayers, CellMapParams};

/// Check that iterator constructors return the right ok or error.
#[test]
fn construction() {
    // Dummy map
    let mut map = CellMap::<TestLayers, f64>::new_from_elem(
        CellMapParams {
            cell_size: Vector2::new(1.0, 1.0),
            cell_bounds: Bounds::new((0, 5), (0, 5)).unwrap(),
            ..Default::default()
        },
        1.0,
    );

    // Try to build new iterators, checking we don't panic
    let _ = map.iter();
    let _ = map.iter_mut();
    assert!(map.window_iter(Vector2::new(2, 2)).is_ok());
    assert!(map.window_iter_mut(Vector2::new(2, 2)).is_ok());

    // Check that window builds return errors when we use a big window
    assert!(map.window_iter(Vector2::new(3, 3)).is_err());
    assert!(map.window_iter_mut(Vector2::new(3, 3)).is_err());
}

#[test]
fn counts() -> Result<(), Error> {
    // Dummy map
    let mut map = CellMap::<TestLayers, f64>::new_from_elem(
        CellMapParams {
            cell_size: Vector2::new(1.0, 1.0),
            cell_bounds: Bounds::new((0, 5), (0, 5)).unwrap(),
            ..Default::default()
        },
        1.0,
    );

    assert_eq!(map.iter().count(), 75);
    assert_eq!(map.iter().layer(TestLayers::Layer0).count(), 25);
    assert_eq!(
        map.iter()
            .layers(&[TestLayers::Layer0, TestLayers::Layer2])
            .count(),
        50
    );
    assert_eq!(map.iter_mut().count(), 75);
    assert_eq!(map.iter_mut().layer(TestLayers::Layer0).count(), 25);
    assert_eq!(
        map.iter_mut()
            .layers(&[TestLayers::Layer0, TestLayers::Layer2])
            .count(),
        50
    );

    assert_eq!(map.window_iter(Vector2::new(1, 1))?.count(), 27);
    assert_eq!(map.window_iter_mut(Vector2::new(1, 1))?.count(), 27);

    assert_eq!(map.window_iter(Vector2::new(2, 2))?.count(), 3);
    assert_eq!(map.window_iter(Vector2::new(2, 2))?.count(), 3);

    Ok(())
}

#[test]
fn window() -> Result<(), Error> {
    // Dummy map
    let map = CellMap::<TestLayers, f64>::new_from_elem(
        CellMapParams {
            cell_size: Vector2::new(1.0, 1.0),
            cell_bounds: Bounds::new((0, 5), (0, 6)).unwrap(),
            ..Default::default()
        },
        1.0,
    );

    // Check shape is correct
    let first = map
        .window_iter(Vector2::new(1, 1))?
        .layer(TestLayers::Layer0)
        .next();

    assert_eq!(first.unwrap().shape(), &[3, 3]);

    // Check the order of cells produced is correct
    let indices: Vec<(usize, usize)> = map
        .window_iter(Vector2::new(1, 1))?
        .layer(TestLayers::Layer0)
        .indexed()
        .map(|((_, idx), _)| (idx.x, idx.y))
        .collect();

    assert_eq!(
        indices,
        vec![
            (1, 1),
            (2, 1),
            (3, 1),
            // (4, 1),
            (1, 2),
            (2, 2),
            (3, 2),
            // (4, 2),
            (1, 3),
            (2, 3),
            (3, 3),
            // (4, 3),
            (1, 4),
            (2, 4),
            (3, 4),
            // (4, 4),
        ]
    );

    Ok(())
}

#[test]
fn line() -> Result<(), Error> {
    // Dummy map
    let map = CellMap::<TestLayers, f64>::new_from_elem(
        CellMapParams {
            cell_size: Vector2::new(1.0, 1.0),
            cell_bounds: Bounds::new((0, 6), (0, 6)).unwrap(),
            ..Default::default()
        },
        1.0,
    );

    // Create new line iterator between two points which should be a straight line
    let indexes: Vec<(usize, usize)> = map
        .line_iter(Point2::new(1.1, 1.1), Point2::new(3.3, 1.1))?
        .layer(TestLayers::Layer0)
        .indexed()
        .map(|((_, i), _)| (i.x, i.y))
        .collect();

    assert_eq!(indexes, vec![(1, 1), (2, 1), (3, 1)]);

    // Create a off diaganal iterator, should produce more points
    let indexes: Vec<(usize, usize)> = map
        .line_iter(Point2::new(1.1, 1.1), Point2::new(4.3, 2.1))?
        .layer(TestLayers::Layer0)
        .indexed()
        .map(|((_, i), _)| (i.x, i.y))
        .collect();

    assert_eq!(indexes, vec![(1, 1), (2, 1), (3, 1), (3, 2), (4, 2)]);

    // Create negative direction vector
    let indexes: Vec<(usize, usize)> = map
        .line_iter(Point2::new(4.3, 2.1), Point2::new(1.1, 1.1))?
        .layer(TestLayers::Layer0)
        .indexed()
        .map(|((_, i), _)| (i.x, i.y))
        .collect();

    assert_eq!(indexes, vec![(4, 2), (3, 2), (3, 1), (2, 1), (1, 1)]);

    Ok(())
}
