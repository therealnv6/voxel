use bevy::{math::Vec3A, prelude::IVec3, render::primitives::HalfSpace};

use crate::chunk::registry::Coordinates;

/// Determines if a 3D point is inside a frustum defined by six half-spaces.
///
/// This function performs frustum culling on a single 3D point using a set of six
/// half-space definitions that together define a frustum. The frustum is typically
/// used in graphics and game development to determine if an object is visible within
/// a specific camera's view volume.
///
/// # Parameters
///
/// - `point`: The 3D point to be tested against the frustum.
/// - `spaces`: An array of six `HalfSpace` instances representing the half-spaces
///    that define the frustum. Each `HalfSpace` includes a normal vector and a
///    distance from the origin. The normal vector points inward to the frustum.
/// - `margin`: A small margin added to the frustum planes to account for potential
///    rounding errors or inaccuracies in the frustum definition. It helps prevent
///    false negatives due to points being slightly outside the frustum.
///
/// # Returns
///
/// A boolean value indicating whether the point is inside the frustum (`true`) or
/// outside the frustum (`false`).
///
/// # Example
///
/// ```
/// let point = Vec3A::new(1.0, 2.0, -3.0);
///
/// let frustum_planes = [
///     HalfSpace::new(/* normal */, /* distance */),
///     HalfSpace::new(/* normal */, /* distance */),
///     // ... more half-spaces ...
/// ];
///
/// let margin = 0.01;
///
/// let is_visible = is_in_frustum(point, frustum_planes, margin);
///
/// if is_visible {
///     println!("Point is visible.");
/// } else {
///     println!("Point is not visible.");
/// }
/// ```
pub fn is_in_frustum(point: impl Into<Vec3A>, spaces: [HalfSpace; 6], margin: f32) -> bool {
    let point = point.into();

    let normals = [
        spaces[0].normal(),
        spaces[1].normal(),
        spaces[2].normal(),
        spaces[3].normal(),
        spaces[4].normal(),
        spaces[5].normal(),
    ];

    let distances = [
        spaces[0].d(),
        spaces[1].d(),
        spaces[2].d(),
        spaces[3].d(),
        spaces[4].d(),
        spaces[5].d(),
    ];

    // perform the frustum culling check with expanded frustum planes
    !(normals[0].dot(point) + (margin + distances[0]) < 0.0)
        && !(normals[1].dot(point) + (margin + distances[1]) < 0.0)
        && !(normals[2].dot(point) + (margin + distances[2]) < 0.0)
        && !(normals[3].dot(point) + (margin + distances[3]) < 0.0)
        && !(normals[4].dot(point) + (margin + distances[4]) < 0.0)
        && !(normals[5].dot(point) + (margin + distances[5]) < 0.0)
}

/// Determines if a batch of points is inside a frustum defined by six half-spaces.
///
/// This function performs frustum culling on a batch of 3D points using a set of six
/// half-space definitions that together define a frustum. The frustum is typically used
/// in graphics and game development to determine if objects are visible within a specific
/// camera's view volume.
///
/// # Parameters
///
/// - `points`: An iterator over items that can be converted into `Vec3A` (3D vectors).
///    These are the points to be tested against the frustum.
/// - `spaces`: An array of six `HalfSpace` instances representing the half-spaces that
///    define the frustum. Each `HalfSpace` includes a normal vector and a distance from
///    the origin. The normal vector points inward to the frustum.
/// - `margin`: A small margin added to the frustum planes to account for potential
///    rounding errors or inaccuracies in the frustum definition. It helps prevent false
///    negatives due to points being slightly outside the frustum.
///
/// # Returns
///
/// An array of boolean values, where each element indicates whether the corresponding
/// point is inside the frustum (`true`) or outside the frustum (`false`).
///
/// # Examples
///
/// Iterating
/// ```
/// let points = vec![
///     Vec3A::new(1.0, 2.0, -3.0),
///     Vec3A::new(0.0, 0.0, -1.0),
///     // ... more points ...
/// ];
///
/// let frustum_planes = [
///     HalfSpace::new(/* normal */, /* distance */),
///     HalfSpace::new(/* normal */, /* distance */),
///     // ... more half-spaces ...
/// ];
///
/// let margin = 0.01;
///
/// let visibility = is_in_frustum_batch::<2>(points, frustum_planes, margin);
///
/// for (index, is_visible) in visibility.iter().enumerate() {
///     if *is_visible {
///         println!("Point {index} is visible.");
///     } else {
///         println!("Point {index} is not visible.");
///     }
/// }
/// ```
///
/// Mapping
/// ```
/// let points = vec![
///     Vec3A::new(1.0, 2.0, -3.0),
///     Vec3A::new(0.0, 0.0, -1.0),
///     // ... more points ...
/// ];
///
/// let frustum_planes = [
///     HalfSpace::new(/* normal */, /* distance */),
///     HalfSpace::new(/* normal */, /* distance */),
///     // ... more half-spaces ...
/// ];
///
/// let margin = 0.01;
///
/// let visibility = is_in_frustum_batch::<2>(points, frustum_planes, margin);
///
/// let is_any_visible = visibility.iter()
///     .filter(|visible| **visible)
///     .next()
///     .is_some();
///
/// println("{is_any_visible}")
/// ```
pub fn is_in_frustum_batch<const SIZE: usize>(
    points: impl IntoIterator<Item = impl Into<Vec3A>>,
    spaces: [HalfSpace; 6],
    margin: f32,
) -> [bool; SIZE] {
    let mut results = [false; SIZE];

    for (index, point) in points.into_iter().enumerate() {
        let point = point.into();
        let result = is_in_frustum(point, spaces, margin);

        results[index] = result;
    }

    results
}

/// Determines if a batch of points is inside a frustum defined by six half-spaces.
///
/// This function performs frustum culling on a batch of 3D points using a set of six
/// half-space definitions that together define a frustum. The frustum is typically used
/// in graphics and game development to determine if objects are visible within a specific
/// camera's view volume. This function automatically uses get_frustum_point_amount() to infer the
/// amount of points that will be used to determine if the object is within the frustum; thus
/// unsized.
///
/// # Parameters
///
/// - `points`: An iterator over items that can be converted into `Vec3A` (3D vectors).
///    These are the points to be tested against the frustum.
/// - `spaces`: An array of six `HalfSpace` instances representing the half-spaces that
///    define the frustum. Each `HalfSpace` includes a normal vector and a distance from
///    the origin. The normal vector points inward to the frustum.
/// - `margin`: A small margin added to the frustum planes to account for potential
///    rounding errors or inaccuracies in the frustum definition. It helps prevent false
///    negatives due to points being slightly outside the frustum.
///
/// # Returns
///
/// An array of boolean values, where each element indicates whether the corresponding
/// point is inside the frustum (`true`) or outside the frustum (`false`).
///
/// # Examples
///
/// Iterating
/// ```
/// let points = vec![
///     Vec3A::new(1.0, 2.0, -3.0),
///     Vec3A::new(0.0, 0.0, -1.0),
///     // ... more points ...
/// ];
///
/// let frustum_planes = [
///     HalfSpace::new(/* normal */, /* distance */),
///     HalfSpace::new(/* normal */, /* distance */),
///     // ... more half-spaces ...
/// ];
///
/// let margin = 0.01;
///
/// let visibility = is_in_frustum_batch_unsized(points, frustum_planes, margin);
///
/// for (index, is_visible) in visibility.iter().enumerate() {
///     if *is_visible {
///         println!("Point {index} is visible.");
///     } else {
///         println!("Point {index} is not visible.");
///     }
/// }
/// ```
///
/// Mapping
/// ```
/// let points = vec![
///     Vec3A::new(1.0, 2.0, -3.0),
///     Vec3A::new(0.0, 0.0, -1.0),
///     // ... more points ...
/// ];
///
/// let frustum_planes = [
///     HalfSpace::new(/* normal */, /* distance */),
///     HalfSpace::new(/* normal */, /* distance */),
///     // ... more half-spaces ...
/// ];
///
/// let margin = 0.01;
///
/// let visibility = is_in_frustum_batch_unsized(points, frustum_planes, margin);
///
/// let is_any_visible = visibility.iter()
///     .filter(|visible| **visible)
///     .next()
///     .is_some();
///
/// println("{is_any_visible}")
/// ```
pub fn is_in_frustum_batch_unsized(
    points: impl IntoIterator<Item = impl Into<Vec3A>>,
    spaces: [HalfSpace; 6],
    margin: f32,
) -> [bool; get_frustum_point_amount()] {
    let mut results = [false; get_frustum_point_amount()];

    for (index, point) in points.into_iter().enumerate() {
        let point = point.into();
        let result = is_in_frustum(point, spaces, margin);

        results[index] = result;
    }

    results
}

pub const fn get_frustum_point_amount() -> usize {
    return 6;
}

/// Creates an array of frustum points based on the given position and dimensions.
///
/// This function calculates six frustum points that define the corners of a frustum.
///
/// # Arguments
///
/// * `pos` - The position of the frustum.
/// * `dimensions` - The dimensions of the frustum (width, height, and depth).
///
/// # Returns
///
/// An array containing six `Vec3A` points that define the frustum corners.
pub fn create_frustum_points(
    IVec3 {
        x: pos_x,
        y: pos_y,
        z: pos_z,
    }: IVec3,
    IVec3 {
        x: width,
        y: height,
        z: depth,
    }: IVec3,
) -> [Vec3A; get_frustum_point_amount()] {
    [
        Coordinates {
            x: pos_x - (width + 1),
            y: pos_y - (height + 1),
            z: pos_z - (depth + 1),
        }
        .as_vec3a(),
        Coordinates {
            x: pos_x + (width + 1),
            y: pos_y + (height + 1),
            z: pos_z + (depth + 1),
        }
        .as_vec3a(),
        Coordinates {
            x: pos_x + (width + 1),
            y: pos_y - (height + 1),
            z: pos_z + (depth + 1),
        }
        .as_vec3a(),
        Coordinates {
            x: pos_x - (width + 1),
            y: pos_y + (height + 1),
            z: pos_z - (depth + 1),
        }
        .as_vec3a(),
        Coordinates {
            x: pos_x + (width + 1),
            y: pos_y + (height + 1),
            z: pos_z - (depth + 1),
        }
        .as_vec3a(),
        Coordinates {
            x: pos_x,
            y: pos_y,
            z: pos_z,
        }
        .as_vec3a(),
    ]
}
