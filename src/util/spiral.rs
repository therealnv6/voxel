pub struct Spiral {
    radius: i32,
}

impl Spiral {
    pub fn new(n: i32) -> Self {
        let r = ((f64::from(n + 1)).sqrt() - 1.0) / 2.0;
        let radius = r.floor() as i32 + 1;
        Spiral { radius }
    }

    pub fn calculate_position(&self, n: i32) -> (i32, i32, i32) {
        let p = (8 * self.radius * (self.radius - 1)) / 2;
        let en = self.radius * 2;
        let a = (1 + n - p) % (self.radius * 8);

        let mut pos = (0, 0, self.radius);
        match a / (self.radius * 2) {
            0 => {
                pos.0 = a - self.radius;
                pos.1 = -self.radius;
            }
            1 => {
                pos.0 = self.radius;
                pos.1 = (a % en) - self.radius;
            }
            2 => {
                pos.0 = self.radius - (a % en);
                pos.1 = self.radius;
            }
            3 => {
                pos.0 = -self.radius;
                pos.1 = self.radius - (a % en);
            }
            _ => (),
        }
        pos
    }
}
