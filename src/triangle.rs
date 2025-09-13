use std::fmt;

/// Three vertex indices of a triangle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Triangle {
    pub first: u32,
    pub second: u32,
    pub third: u32,
}

impl Triangle {
    pub fn new(first: u32, second: u32, third: u32) -> Self {
        Self { first, second, third }
    }

    pub fn rotate(self) -> Self {
        Self {
            first: self.second,
            second: self.third,
            third: self.first,
        }
    }

    pub fn neighbours(&self, other: &Triangle, match_mid: bool) -> bool {
        if match_mid {
            self.second == other.second && self.third == other.first
        } else {
            self.first == other.first && self.third == other.second
        }
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.first, self.second, self.third)
    }
}

impl From<&[u32]> for Triangle {
    fn from(value: &[u32]) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclical_rotation() {
        let triangle = Triangle::new(0, 1, 2);

        assert_eq!(triangle, triangle.rotate().rotate().rotate());
    }

    #[test]
    fn neighbouring() {
        let a = Triangle::new(0, 1, 2);
        let b = Triangle::new(2, 1, 3);
        let c = Triangle::new(0, 2, 3);

        assert!(a.neighbours(&b, true));
        assert!(a.neighbours(&c, false));
    }

    #[test]
    fn not_neighbouring() {
        let a = Triangle::new(0, 1, 2);
        let b = Triangle::new(2, 1, 3);
        let c = Triangle::new(0, 2, 3);

        assert!(!a.neighbours(&b, false));
        assert!(!a.neighbours(&c, true));
    }
}

