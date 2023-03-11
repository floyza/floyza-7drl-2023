use bracket_lib::terminal::Point;

pub fn normalize_pt(pt: Point) -> Point {
    let x_neg = pt.x < 0;
    let y_neg = pt.y < 0;
    let mut x = pt.x.abs();
    let mut y = pt.y.abs();
    if x > y {
        y = (y as f32 / x as f32).round() as i32;
        x = 1;
    } else {
        x = (x as f32 / y as f32).round() as i32;
        y = 1;
    }
    if x_neg {
        x = -x;
    }
    if y_neg {
        y = -y;
    }
    return Point::new(x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_pt_test() {
        assert_eq!(normalize_pt(Point::new(5, 4)), Point::new(1, 1));
        assert_eq!(normalize_pt(Point::new(5, 2)), Point::new(1, 0));
        assert_eq!(normalize_pt(Point::new(5, 2)), Point::new(1, 0));
        assert_eq!(normalize_pt(Point::new(3, -5)), Point::new(1, -1));
    }
}
