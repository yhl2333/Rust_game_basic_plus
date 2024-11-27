use crate::prelude::{Point, Point3};
use std::cmp::{max, min};

/// Enumeration of available 2D Distance algorithms
#[derive(Clone, Copy)]
pub enum DistanceAlg {
    /// Use the Pythagoras algorithm for determining distance - sqrt(A^2 + B^2)
    Pythagoras,
    /// Us the Pythagoras algorithm for distance, but omitting the square-root for a faster but squared result.
    PythagorasSquared,
    /// Use Manhattan distance (distance up plus distance along)
    Manhattan,
    /// Use Chebyshev distance (like Manhattan, but adds one to each entry)
    Chebyshev,
    /// Use a diagonal distance, the max of the x and y distances
    Diagonal,
}

impl DistanceAlg {
    /// Provides a 2D distance between points, using the specified algorithm.
    pub fn distance2d(self, start: Point, end: Point) -> f32 {
        match self {
            DistanceAlg::Pythagoras => distance2d_pythagoras(start, end),
            DistanceAlg::PythagorasSquared => distance2d_pythagoras_squared(start, end),
            DistanceAlg::Manhattan => distance2d_manhattan(start, end),
            DistanceAlg::Chebyshev => distance2d_chebyshev(start, end),
            DistanceAlg::Diagonal => distance2d_diagonal(start, end),
        }
    }
    /// Provides a 3D distance between points, using the specified algorithm.
    pub fn distance3d(self, start: Point3, end: Point3) -> f32 {
        match self {
            DistanceAlg::Pythagoras => distance3d_pythagoras(start, end),
            DistanceAlg::PythagorasSquared => distance3d_pythagoras_squared(start, end),
            DistanceAlg::Manhattan => distance3d_manhattan(start, end),
            DistanceAlg::Chebyshev => distance3d_pythagoras(start, end),
            DistanceAlg::Diagonal => distance3d_diagonal(start, end),
        }
    }
}

/// Calculates a Pythagoras distance between two points, and skips the square root for speed.
fn distance2d_pythagoras_squared(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    (dx * dx) + (dy * dy)
}

/// Calculates a Manhattan distance between two points
fn distance2d_manhattan(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    dx + dy
}

/// Calculates a Manhattan distance between two 3D points
fn distance3d_manhattan(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    dx + dy + dz
}

/// Calculates a Chebyshev distance between two points
/// See: http://theory.stanford.edu/~amitp/GameProgramming/Heuristics.html
fn distance2d_chebyshev(start: Point, end: Point) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    if dx > dy {
        (dx - dy) + 1.0 * dy
    } else {
        (dy - dx) + 1.0 * dx
    }
}

/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras_squared(start: Point3, end: Point3) -> f32 {
    let dx = (max(start.x, end.x) - min(start.x, end.x)) as f32;
    let dy = (max(start.y, end.y) - min(start.y, end.y)) as f32;
    let dz = (max(start.z, end.z) - min(start.z, end.z)) as f32;
    (dx * dx) + (dy * dy) + (dz * dz)
}

/// Calculates a Pythagoras distance between two points.
fn distance2d_pythagoras(start: Point, end: Point) -> f32 {
    let dsq = distance2d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

/// Calculates a Pythagoras distance between two 3D points.
fn distance3d_pythagoras(start: Point3, end: Point3) -> f32 {
    let dsq = distance3d_pythagoras_squared(start, end);
    f32::sqrt(dsq)
}

// Calculates a diagonal distance
fn distance2d_diagonal(start: Point, end: Point) -> f32 {
    i32::max((start.x - end.x).abs(), (start.y - end.y).abs()) as f32
}

fn distance3d_diagonal(start: Point3, end: Point3) -> f32 {
    i32::max(
        (start.x - end.x).abs(),
        i32::max((start.y - end.y).abs(), (start.z - end.z).abs()),
    ) as f32
}

#[cfg(test)]
mod tests {
    use crate::prelude::{DistanceAlg, Point, Point3};

    #[test]
    fn test_pythagoras_distance() {
        let mut d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 7.071_068) < std::f32::EPSILON);
    }

    #[test]
    fn test_pythagoras_distance3d() {
        let mut d = DistanceAlg::Pythagoras.distance3d(Point3::new(0, 0, 0), Point3::new(5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance3d(Point3::new(0, 0, 0), Point3::new(-5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Pythagoras.distance3d(Point3::new(0, 0, 0), Point3::new(5, 5, 5));
        assert!(f32::abs(d - 8.660_254_5) < std::f32::EPSILON);
    }

    #[test]
    fn test_pythagoras_squared_distance() {
        let mut d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 50.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_pythagoras_squared_distance3d() {
        let mut d =
            DistanceAlg::PythagorasSquared.distance3d(Point3::new(0, 0, 0), Point3::new(5, 0, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance3d(Point3::new(0, 0, 0), Point3::new(-5, 0, 0));
        assert!(f32::abs(d - 25.0) < std::f32::EPSILON);

        d = DistanceAlg::PythagorasSquared.distance3d(Point3::new(0, 0, 0), Point3::new(5, 5, 5));
        assert!(f32::abs(d - 75.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_manhattan_distance() {
        let mut d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 10.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_manhattan_distance3d() {
        let mut d = DistanceAlg::Manhattan.distance3d(Point3::new(0, 0, 0), Point3::new(5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance3d(Point3::new(0, 0, 0), Point3::new(-5, 0, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Manhattan.distance3d(Point3::new(0, 0, 0), Point3::new(5, 5, 5));
        assert!(f32::abs(d - 15.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_chebyshev_distance() {
        let mut d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(-5, 0));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(0, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(0, -5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        d = DistanceAlg::Chebyshev.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);
    }

    #[test]
    fn test_algorithm_from_shared_reference() {
        let mut algorithm = DistanceAlg::Chebyshev;
        let mut d = algorithm.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 5.0) < std::f32::EPSILON);

        algorithm = DistanceAlg::Manhattan;
        d = algorithm.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 10.0) < std::f32::EPSILON);

        let shared_ref = &algorithm;
        d = shared_ref.distance2d(Point::new(0, 0), Point::new(5, 5));
        assert!(f32::abs(d - 10.0) < std::f32::EPSILON);
    }
}
