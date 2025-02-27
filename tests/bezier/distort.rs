use flo_curves::bezier::{
    distort_curve, walk_curve_evenly, BezierCurve, BezierCurveFactory, Coord2, Coordinate2D,
    Coordinate3D, Curve,
};

#[test]
fn line_to_sine_wave() {
    let line = Curve::from_points(
        Coord2(100.0, 100.0),
        (Coord2(100.0, 100.0), Coord2(400.0, 100.0)),
        Coord2(400.0, 100.0),
    );
    let distorted = distort_curve::<_, _, Curve<_>>(
        &line,
        |pos, _t| Coord2(pos.x(), pos.y() + (pos.x() * 20.0).sin()),
        1.0,
        1.0,
    )
    .expect("Fit curve");

    for curve in distorted.into_iter() {
        println!("{:?}", curve);

        for section in walk_curve_evenly(&curve, 1.0, 0.1) {
            let (t_min, t_max) = section.original_curve_t_values();
            let t_mid = (t_min + t_max) / 2.0;
            let pos = section.point_at_pos(t_mid);

            let expected_y = 100.0 + (pos.x() * 20.0).sin();
            let actual_y = pos.y();

            println!(
                "  {:?} {:?} {:?} {:?}",
                t_mid,
                expected_y,
                actual_y,
                (expected_y - actual_y).abs()
            );

            assert!((expected_y - actual_y).abs() < 4.0);
        }
    }
}
