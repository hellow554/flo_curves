use flo_curves::bezier::{BezierCurve, Coord2, Coordinate, Curve};
use flo_curves::line::{line_to_bezier, Line2D};

#[test]
fn convert_line_to_bezier_curve() {
    let line = (Coord2(10.0, 20.0), Coord2(40.0, 30.0));
    let curve = line_to_bezier::<_, Curve<_>>(&line);

    assert!(curve.start_point == Coord2(10.0, 20.0));
    assert!(curve.end_point == Coord2(40.0, 30.0));
    assert!(curve.control_points.0.distance_to(&Coord2(20.0, 23.33)) < 0.1);
    assert!(curve.control_points.1.distance_to(&Coord2(30.0, 26.66)) < 0.1);
}
