use flo_curves::bezier;

#[test]
fn search_for_x_coordinate() {
    // Initial curve
    let (w1, w2, w3, w4) = (1.0, -2.0, 3.0, 4.0);

    // Search for the t value for a particular X coord
    let x_coord = 1.5;
    let matching_values =
        bezier::search_bounds4(0.01, w1, w2, w3, w4, |p1, p2| p1 < x_coord && p2 > x_coord);

    // Should be only 1 coordinate with this curve
    assert!(matching_values.len() == 1);

    // Basis function should be within 0.01
    let actual_val = bezier::basis(matching_values[0], w1, w2, w3, w4);
    assert!((actual_val - x_coord).abs() < 0.01);
}

#[test]
fn coordinate_outside_curve_produces_no_results() {
    // Initial curve
    let (w1, w2, w3, w4) = (1.0, -2.0, 3.0, 4.0);

    // Search for the t value for a particular X coord, which is outside the curve
    let x_coord = 5.0;
    let matching_values =
        bezier::search_bounds4(0.01, w1, w2, w3, w4, |p1, p2| p1 < x_coord && p2 > x_coord);

    // No points on the curve match this coordinate
    assert!(matching_values.is_empty());
}
