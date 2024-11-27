use crate::prelude::{Bresenham, DistanceAlg, Point};

/// Enumeration of available 2D Distance algorithms
pub enum LineAlg {
    /// Use Bresenham's rasterization algorithm for line definition
    Bresenham,
    /// Use a vector approach to line solving.
    Vector,
}

/// Plots a line between two 2D points and returns a vector of points along the line.
pub fn line2d(algorithm: LineAlg, start: Point, end: Point) -> Vec<Point> {
    match algorithm {
        LineAlg::Bresenham => line2d_bresenham(start, end),
        LineAlg::Vector => line2d_vector(start, end),
    }
}

/// Uses a Bresenham's algorithm to plot a line between two points. On some CPUs, this is faster
/// than Bresenham.
pub fn line2d_bresenham(start: Point, end: Point) -> Vec<Point> {
    let line = Bresenham::new(start, end);
    line.chain(std::iter::once(end)).collect()
}

/// Uses a 2D vector algorithm to plot a line between two points. On some CPUs, this is faster
/// than Bresenham.
pub fn line2d_vector(start: Point, end: Point) -> Vec<Point> {
    use ultraviolet::Vec2;

    if start == end {
        return vec![start];
    }

    let mut pos = Vec2::new(start.x as f32, start.y as f32);
    let dest = Vec2::new(end.x as f32, end.y as f32);
    let n_steps = DistanceAlg::Pythagoras.distance2d(start, end);
    let slope = (dest - pos).normalized();
    let mut result: Vec<Point> = Vec::with_capacity(n_steps as usize + 1);
    let mut count = 0;
    result.push(start);
    loop {
        pos += slope;
        let new_point = Point::new(pos.x.round() as i32, pos.y.round() as i32);
        if result[result.len() - 1] != new_point {
            if count == n_steps as i32 {
                result.push(end);
                break;
            }
            result.push(new_point);
            if new_point == end {
                // arrived
                break;
            }
        }
        count += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::prelude::{line2d_bresenham, line2d_vector, Point};

    #[test]
    fn vector_line_h() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(5, 0);
        let line = line2d_vector(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0)
            ]
        );
    }

    #[test]
    fn vector_line_v() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, 5);
        let line = line2d_vector(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
                Point::new(0, 4),
                Point::new(0, 5)
            ]
        );
    }

    #[test]
    fn vector_line_v_neg() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, -5);
        let line = line2d_vector(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(0, -2),
                Point::new(0, -3),
                Point::new(0, -4),
                Point::new(0, -5)
            ]
        );
    }

    #[test]
    fn bresenham_line_h() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(5, 0);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(2, 0),
                Point::new(3, 0),
                Point::new(4, 0),
                Point::new(5, 0)
            ]
        );
    }

    #[test]
    fn bresenham_line_v() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, 5);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, 1),
                Point::new(0, 2),
                Point::new(0, 3),
                Point::new(0, 4),
                Point::new(0, 5)
            ]
        );
    }

    #[test]
    fn bresenham_line_v_neg() {
        let pt = Point::new(0, 0);
        let pt2 = Point::new(0, -5);
        let line = line2d_bresenham(pt, pt2);
        assert_eq!(
            line,
            vec![
                Point::new(0, 0),
                Point::new(0, -1),
                Point::new(0, -2),
                Point::new(0, -3),
                Point::new(0, -4),
                Point::new(0, -5)
            ]
        );
    }

    #[test]
    pub fn infinite_loop_bug181() {
        let pt = Point { x: 2, y: 2 };
        let pt2 = Point { x: 7, y: -4 };
        let line = line2d_vector(pt, pt2);

        assert_eq!(line[line.len() - 1], pt2);
    }
}
