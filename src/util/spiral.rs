/// A struct representing an iterator that generates coordinates in a spiral pattern.
pub struct SpiralIterator {
    center_chunk_x: i32,
    center_chunk_z: i32,
    radius: i32,
    x: i32,
    z: i32,
    dx: i32,
    dz: i32,
    max_radius: i32,
}

impl SpiralIterator {
    /// Creates a new `SpiralIterator` instance.
    ///
    /// The iterator generates coordinates in a spiral pattern centered around the given chunk
    /// coordinates (`center_chunk_x` and `center_chunk_z`), up to a maximum distance (`radius`)
    /// from the center.
    ///
    /// # Arguments
    ///
    /// * `center_chunk_x` - The x-coordinate of the center chunk.
    /// * `center_chunk_z` - The z-coordinate of the center chunk.
    /// * `radius` - The maximum distance from the center in chunks.
    ///
    /// # Example
    ///
    /// ```
    /// let center_x = 0;
    /// let center_z = 0;
    /// let radius = 2;
    /// let iterator = SpiralIterator::new(center_x, center_z, radius);
    /// ```
    pub fn new(center_chunk_x: i32, center_chunk_z: i32, radius: i32) -> Self {
        SpiralIterator {
            center_chunk_x,
            center_chunk_z,
            radius,
            x: 0,
            z: 0,
            dx: 0,
            dz: -1,
            max_radius: radius * radius,
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (i32, i32);

    /// Advances the iterator and returns the next coordinate in the spiral pattern.
    ///
    /// Returns `Some((x, z))` if the next coordinate is within the specified radius.
    /// Returns `None` when all coordinates within the radius have been generated.
    fn next(&mut self) -> Option<Self::Item> {
        // check if the distance from the origin exceeds the maximum radius
        if self.x * self.x + self.z * self.z > self.max_radius {
            return None;
        }

        // check if the current position is within the defined radius bounds
        if -self.radius <= self.x
            && self.x <= self.radius
            && -self.radius <= self.z
            && self.z <= self.radius
        {
            // calculate the result coordinates based on the center and current position
            let result = (self.center_chunk_x + self.x, self.center_chunk_z + self.z);

            // update the movement direction if specific conditions are met
            if self.x == self.z
                || (self.x < 0 && self.x == -self.z)
                || (self.x > 0 && self.x == 1 - self.z)
            {
                // swap dx and dz and negate dx
                let temp = self.dx;
                self.dx = -self.dz;
                self.dz = temp;
            }

            // move the current position based on the calculated direction
            self.x += self.dx;
            self.z += self.dz;

            Some(result) // return the calculated result
        } else {
            None // current position is outside radius bounds, return None
        }
    }
}
