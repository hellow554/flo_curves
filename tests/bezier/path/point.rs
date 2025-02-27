use flo_curves::arc::Circle;
use flo_curves::bezier::path::{path_contains_point, SimpleBezierPath};
use flo_curves::Coord2;

#[test]
fn simple_path_contains_point() {
    // Path is a square
    let path = (
        Coord2(1.0, 2.0),
        vec![
            (Coord2(3.0, 2.0), Coord2(6.0, 2.0), Coord2(9.0, 2.0)),
            (Coord2(9.0, 4.0), Coord2(9.0, 6.0), Coord2(9.0, 8.0)),
            (Coord2(6.0, 8.0), Coord2(3.0, 8.0), Coord2(1.0, 8.0)),
            (Coord2(1.0, 6.0), Coord2(1.0, 4.0), Coord2(1.0, 2.0)),
        ],
    );

    // Point should be inside
    assert!(path_contains_point(&path, &Coord2(5.0, 5.0)));
    assert!(path_contains_point(&path, &Coord2(3.0, 4.0)));
}

#[test]
fn circle_contains_point() {
    // Path is a circle
    let path: SimpleBezierPath = Circle::new(Coord2(5.0, 5.0), 4.0).to_path();

    // Point should be inside
    assert!(path_contains_point(&path, &Coord2(5.0, 5.0)));
    assert!(path_contains_point(&path, &Coord2(3.0, 4.0)));
    assert!(path_contains_point(&path, &Coord2(7.5, 7.5)));
    assert!(path_contains_point(&path, &Coord2(2.5, 7.5)));
}

#[test]
fn circle_edge_is_inside() {
    // Path is a circle
    let path: SimpleBezierPath = Circle::new(Coord2(5.0, 5.0), 4.0).to_path();

    // Points on the edge of the circle should be inside
    assert!(path_contains_point(&path, &Coord2(8.8, 5.0)));
    assert!(path_contains_point(&path, &Coord2(8.9, 5.0)));
    assert!(path_contains_point(&path, &Coord2(8.99, 5.0)));
    assert!(path_contains_point(&path, &Coord2(8.999, 5.0)));

    assert!(path_contains_point(&path, &Coord2(5.0, 8.8)));
    assert!(path_contains_point(&path, &Coord2(5.0, 8.9)));
    assert!(path_contains_point(&path, &Coord2(5.0, 8.99)));

    assert!(path_contains_point(&path, &Coord2(1.2, 5.0)));
    assert!(path_contains_point(&path, &Coord2(1.1, 5.0)));
    assert!(path_contains_point(&path, &Coord2(1.01, 5.0)));

    assert!(path_contains_point(&path, &Coord2(5.0, 1.2)));
    assert!(path_contains_point(&path, &Coord2(5.0, 1.1)));
    assert!(path_contains_point(&path, &Coord2(5.0, 1.01)));
}

#[test]
fn point_on_edge_is_not_in_path() {
    // Path is a square
    let path = (
        Coord2(1.0, 2.0),
        vec![
            (Coord2(3.0, 2.0), Coord2(6.0, 2.0), Coord2(9.0, 2.0)),
            (Coord2(9.0, 4.0), Coord2(9.0, 6.0), Coord2(9.0, 8.0)),
            (Coord2(6.0, 8.0), Coord2(3.0, 8.0), Coord2(1.0, 8.0)),
            (Coord2(1.0, 6.0), Coord2(1.0, 4.0), Coord2(1.0, 2.0)),
        ],
    );

    // Points just on the boundary should be outside of the path
    assert!(!path_contains_point(&path, &Coord2(5.0, 2.0)));
    assert!(!path_contains_point(&path, &Coord2(1.0, 4.0)));
}

#[test]
fn corner_is_in_path() {
    // Path is a square
    let path = (
        Coord2(1.0, 2.0),
        vec![
            (Coord2(3.0, 2.0), Coord2(6.0, 2.0), Coord2(9.0, 2.0)),
            (Coord2(9.0, 4.0), Coord2(9.0, 6.0), Coord2(9.0, 8.0)),
            (Coord2(6.0, 8.0), Coord2(3.0, 8.0), Coord2(1.0, 8.0)),
            (Coord2(1.0, 6.0), Coord2(1.0, 4.0), Coord2(1.0, 2.0)),
        ],
    );

    // Points right on the edge but on the corners are in the path
    assert!(path_contains_point(&path, &Coord2(1.001, 2.001)));
    assert!(path_contains_point(&path, &Coord2(8.999, 2.001)));
    assert!(path_contains_point(&path, &Coord2(1.001, 7.999)));
    assert!(path_contains_point(&path, &Coord2(8.999, 7.999)));
}

#[test]
fn points_outside_bounds_are_outside_path() {
    // Path is a square
    let path = (
        Coord2(1.0, 2.0),
        vec![
            (Coord2(3.0, 2.0), Coord2(6.0, 2.0), Coord2(9.0, 2.0)),
            (Coord2(9.0, 4.0), Coord2(9.0, 6.0), Coord2(9.0, 8.0)),
            (Coord2(6.0, 8.0), Coord2(3.0, 8.0), Coord2(1.0, 8.0)),
            (Coord2(1.0, 6.0), Coord2(1.0, 4.0), Coord2(1.0, 2.0)),
        ],
    );

    // Points far outside the path should be outside
    assert!(!path_contains_point(&path, &Coord2(5.0, 20.0)));
    assert!(!path_contains_point(&path, &Coord2(5.0, -5.0)));
    assert!(!path_contains_point(&path, &Coord2(20.0, 5.0)));
    assert!(!path_contains_point(&path, &Coord2(-5.0, 5.0)));
    assert!(!path_contains_point(&path, &Coord2(3.0, 20.0)));
}

#[test]
fn circle_edges_do_not_contain_point() {
    // Path is a circle
    let path: SimpleBezierPath = Circle::new(Coord2(5.0, 5.0), 4.0).to_path();

    // Points should be inside the bounds but not in the circle
    assert!(!path_contains_point(&path, &Coord2(8.5, 8.5)));
    assert!(!path_contains_point(&path, &Coord2(1.4, 1.5)));
}

#[test]
fn crossing_first_point_leaves_us_outside_circle() {
    // Path is a circle
    let path: SimpleBezierPath = Circle::new(Coord2(5.0, 5.0), 4.0).to_path();

    // This line crosses the first point of the circle (which can appear as a crossing at both the start and end of the path, which might fool the algorithm into thinking the point is inside)
    assert!(!path_contains_point(&path, &Coord2(1.5, 1.5)));
}
