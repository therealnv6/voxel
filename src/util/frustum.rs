use bevy::{math::Vec3A, render::primitives::HalfSpace};

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

    // Perform the frustum culling check with expanded frustum planes
    let is_visible = !(normals[0].dot(point) + distances[0] < 0.0 + margin)
        && !(normals[1].dot(point) + distances[1] < 0.0 + margin)
        && !(normals[2].dot(point) + distances[2] < 0.0 + margin)
        && !(normals[3].dot(point) + distances[3] < 0.0 + margin)
        && !(normals[4].dot(point) + distances[4] < 0.0 + margin)
        && !(normals[5].dot(point) + distances[5] < 0.0 + margin);

    is_visible
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
/// let visibility = is_in_frustum_batch(points, frustum_planes, margin);
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
/// let visibility = is_in_frustum_batch(points, frustum_planes, margin);
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
