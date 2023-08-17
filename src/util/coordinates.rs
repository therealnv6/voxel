use crate::chunk::registry::ChunkRegistry;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinates(pub i32, pub i32);

impl From<i32> for Coordinates {
    fn from(val: i32) -> Self {
        let [x, y] = ChunkRegistry::id_to_domain(val);
        Coordinates(x, y)
    }
}

impl From<(i32, i32)> for Coordinates {
    fn from(val: (i32, i32)) -> Self {
        Coordinates(val.0, val.1)
    }
}

impl From<[i32; 2]> for Coordinates {
    fn from(val: [i32; 2]) -> Self {
        Coordinates(val[0], val[1])
    }
}

impl Into<(i32, i32)> for Coordinates {
    fn into(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

impl std::ops::Add<Coordinates> for Coordinates {
    type Output = Coordinates;

    fn add(self, other: Coordinates) -> Coordinates {
        Coordinates(self.0 + other.0, self.1 + other.1)
    }
}
