use flo_curves::arc::Circle;
use flo_curves::bezier::path::{
    BezierPath, BezierPathBuilder, BezierPathFactory, GraphEdge, GraphPath, GraphPathEdgeKind,
    GraphRayCollision, PathDirection, PathLabel, SimpleBezierPath,
};
use flo_curves::{BezierCurve, BoundingBox, Coord2, Coordinate, Coordinate2D, Coordinate3D, Line};

use std::f64;

#[test]
pub fn create_and_read_simple_graph_path() {
    let path = (
        Coord2(10.0, 11.0),
        vec![
            (Coord2(15.0, 16.0), Coord2(17.0, 18.0), Coord2(19.0, 20.0)),
            (Coord2(21.0, 22.0), Coord2(23.0, 24.0), Coord2(25.0, 26.0)),
        ],
    );
    let graph_path = GraphPath::from_path(&path, ());

    assert!(graph_path.num_points() == 3);

    // Point 0 edges
    {
        let edges = graph_path.edges_for_point(0).collect::<Vec<_>>();

        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Uncategorised);
        assert!(edges[0].start_point() == Coord2(10.0, 11.0));
        assert!(edges[0].end_point() == Coord2(19.0, 20.0));
        assert!(edges[0].control_points() == (Coord2(15.0, 16.0), Coord2(17.0, 18.0)));
    }

    // Point 1 edges
    {
        let edges = graph_path.edges_for_point(1).collect::<Vec<_>>();

        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Uncategorised);
        assert!(edges[0].start_point() == Coord2(19.0, 20.0));
        assert!(edges[0].end_point() == Coord2(25.0, 26.0));
        assert!(edges[0].control_points() == (Coord2(21.0, 22.0), Coord2(23.0, 24.0)));
    }

    // Point 2 edges
    {
        let edges = graph_path.edges_for_point(2).collect::<Vec<_>>();
        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Uncategorised);
        assert!(edges[0].start_point() == Coord2(25.0, 26.0));
        assert!(edges[0].end_point() == Coord2(10.0, 11.0));
    }
}

#[test]
pub fn create_and_read_simple_graph_path_reverse() {
    let path = (
        Coord2(10.0, 11.0),
        vec![
            (Coord2(15.0, 16.0), Coord2(17.0, 18.0), Coord2(19.0, 20.0)),
            (Coord2(21.0, 22.0), Coord2(23.0, 24.0), Coord2(25.0, 26.0)),
        ],
    );
    let graph_path = GraphPath::from_path(&path, ());

    assert!(graph_path.num_points() == 3);

    // Point 0 edges
    {
        let edges = graph_path.reverse_edges_for_point(0).collect::<Vec<_>>();

        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Uncategorised);
        assert!(edges[0].start_point() == Coord2(10.0, 11.0));
        assert!(edges[0].end_point() == Coord2(25.0, 26.0));
        assert!(edges[0].control_points() == (Coord2(19.9, 20.9), Coord2(14.95, 15.95)));
    }

    // Point 1 edges
    {
        let edges = graph_path.reverse_edges_for_point(1).collect::<Vec<_>>();

        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Uncategorised);
        assert!(edges[0].start_point() == Coord2(19.0, 20.0));
        assert!(edges[0].end_point() == Coord2(10.0, 11.0));
        assert!(edges[0].control_points() == (Coord2(17.0, 18.0), Coord2(15.0, 16.0)));
    }

    // Point 2 edges
    {
        let edges = graph_path.reverse_edges_for_point(2).collect::<Vec<_>>();
        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Uncategorised);
        assert!(edges[0].start_point() == Coord2(25.0, 26.0));
        assert!(edges[0].end_point() == Coord2(19.0, 20.0));
        assert!(edges[0].control_points() == (Coord2(23.0, 24.0), Coord2(21.0, 22.0)));
    }
}

#[test]
pub fn collide_two_rectangles() {
    // Create the two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 4.0))
        .line_to(Coord2(9.0, 4.0))
        .line_to(Coord2(9.0, 9.0))
        .line_to(Coord2(4.0, 9.0))
        .line_to(Coord2(4.0, 4.0))
        .build();

    let rectangle1 = GraphPath::from_path(&rectangle1, 1);
    let rectangle2 = GraphPath::from_path(&rectangle2, 2);

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.1);

    // 10 points in the collision
    assert!(collision.num_points() == 10);

    let mut check_count = 0;

    for point_idx in 0..10 {
        // Check the edges for each point
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();

        assert!(edges.len() <= 2);
        assert!(!edges.is_empty());

        assert!(edges[0].kind() == GraphPathEdgeKind::Uncategorised);
        assert!(edges.len() == 1 || edges[1].kind() == GraphPathEdgeKind::Uncategorised);

        // Edges leading up to the collision
        if edges[0].start_point() == Coord2(5.0, 1.0) {
            check_count += 1;

            assert!(edges.len() == 1);
            assert!(edges[0].end_point().distance_to(&Coord2(5.0, 4.0)) < 0.1);
            assert!(edges.iter().all(|edge| edge.label() == 1));
        }

        if edges[0].start_point() == Coord2(5.0, 5.0) {
            check_count += 1;

            assert!(edges.len() == 1);
            assert!(edges[0].end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1);
            assert!(edges.iter().all(|edge| edge.label() == 1));
        }

        if edges[0].start_point() == Coord2(1.0, 5.0) {
            check_count += 1;

            assert!(edges.len() == 1);
            assert!(edges[0].end_point().distance_to(&Coord2(1.0, 1.0)) < 0.1);
            assert!(edges.iter().all(|edge| edge.label() == 1));
        }

        if edges[0].start_point() == Coord2(4.0, 4.0) {
            check_count += 1;

            assert!(edges.len() == 1);
            assert!(edges[0].end_point().distance_to(&Coord2(5.0, 4.0)) < 0.1);
            assert!(edges.iter().all(|edge| edge.label() == 2));
        }

        // Collision edges
        if edges[0].start_point().distance_to(&Coord2(4.0, 5.0)) < 0.1 {
            check_count += 1;

            assert!(edges.len() == 2);
            assert!(edges
                .iter()
                .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 4.0)) < 0.1));
            assert!(edges
                .iter()
                .any(|edge| edge.end_point().distance_to(&Coord2(1.0, 5.0)) < 0.1));
            assert!(edges.iter().any(|edge| edge.label() == 1));
            assert!(edges.iter().any(|edge| edge.label() == 2));
        }

        if edges[0].start_point().distance_to(&Coord2(5.0, 4.0)) < 0.1 {
            check_count += 1;

            assert!(edges.len() == 2);
            assert!(edges
                .iter()
                .any(|edge| edge.end_point().distance_to(&Coord2(9.0, 4.0)) < 0.1));
            assert!(edges
                .iter()
                .any(|edge| edge.end_point().distance_to(&Coord2(5.0, 5.0)) < 0.1));
            assert!(edges.iter().any(|edge| edge.label() == 1));
            assert!(edges.iter().any(|edge| edge.label() == 2));
        }
    }

    // Checked 6 (of 10) edges
    assert!(check_count == 6);
}

#[test]
pub fn collide_identical_rectangles() {
    // Create the two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = rectangle1.clone();

    let rectangle1 = GraphPath::from_path(&rectangle1, 1);
    let rectangle2 = GraphPath::from_path(&rectangle2, 2);

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.1);

    for point_idx in 0..(collision.num_points()) {
        // Check the edges for each point
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();
        println!("Point {:?} edges: {:?}", point_idx, edges);
    }

    // 4 points in the collision
    assert!(collision.num_points() == 8);

    for point_idx in 0..8 {
        // Check the edges for each point
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();

        // 4 of the points have 2 edges, 4 of the points have 0 edges
        if point_idx < 4 {
            assert!(edges.len() == 2);
        } else {
            assert!(edges.is_empty());
        }
    }
}

#[test]
fn multiple_collisions_on_one_edge() {
    // Create the two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(2.0, 0.0))
        .line_to(Coord2(2.0, 6.0))
        .line_to(Coord2(4.0, 6.0))
        .line_to(Coord2(4.0, 0.0))
        .line_to(Coord2(2.0, 0.0))
        .build();

    let rectangle1 = GraphPath::from_path(&rectangle1, ());
    let rectangle2 = GraphPath::from_path(&rectangle2, ());

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.1);

    // 12 points in the collision
    assert!(collision.num_points() == 12);

    // Check the intersection points
    for point_idx in 0..12 {
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();

        assert!(edges.len() <= 2);
        if edges.len() == 2 {
            if edges[0].start_point().distance_to(&Coord2(2.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 5.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(1.0, 1.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 0.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(2.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 6.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(5.0, 5.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 1.0)) < 0.1));
            } else {
                // These are the only four intersection points that should exist
                println!("{:?}", edges[0].start_point());
                assert!(false)
            }
        }
    }
}

#[test]
fn multiple_collisions_on_one_edge_opposite_direction() {
    // Create the two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 0.0))
        .line_to(Coord2(4.0, 6.0))
        .line_to(Coord2(2.0, 6.0))
        .line_to(Coord2(2.0, 0.0))
        .line_to(Coord2(4.0, 0.0))
        .build();

    let rectangle1 = GraphPath::from_path(&rectangle1, ());
    let rectangle2 = GraphPath::from_path(&rectangle2, ());

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.1);

    // 12 points in the collision
    assert!(collision.num_points() == 12);

    // Check the intersection points
    let mut num_intersects = 0;
    for point_idx in 0..12 {
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();

        assert!(edges.len() <= 2);
        assert!(!edges.is_empty());
        if edges.len() == 2 {
            num_intersects += 1;

            if edges[0].start_point().distance_to(&Coord2(2.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 0.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(1.0, 1.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(2.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(5.0, 5.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 6.0)) < 0.1));
            } else {
                // These are the only four intersection points that should exist
                println!("{:?}", edges[0].start_point());
                assert!(false)
            }
        } else if edges.len() == 1 {
            let edge = edges.iter().nth(0).unwrap();
            let start_point = edge.start_point();

            assert!(
                (start_point.x() - 1.0).abs() < 0.01
                    || (start_point.x() - 5.0).abs() < 0.01
                    || (start_point.x() - 2.0).abs() < 0.01
                    || (start_point.x() - 4.0).abs() < 0.01
            );
            assert!(
                (start_point.y() - 1.0).abs() < 0.01
                    || (start_point.y() - 5.0).abs() < 0.01
                    || (start_point.y() - 0.0).abs() < 0.01
                    || (start_point.y() - 6.0).abs() < 0.01
            );
        }
    }

    assert!(num_intersects == 4);
}

#[test]
fn collision_at_same_point() {
    // Two rectangles, with the collision point already subdivided
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(2.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 0.0))
        .line_to(Coord2(4.0, 6.0))
        .line_to(Coord2(2.0, 6.0))
        .line_to(Coord2(2.0, 1.0))
        .line_to(Coord2(2.0, 0.0))
        .line_to(Coord2(4.0, 0.0))
        .build();

    let rectangle1 = GraphPath::from_path(&rectangle1, ());
    let rectangle2 = GraphPath::from_path(&rectangle2, ());

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.05);

    // 12 points in the collision (but we can allow for the shared point to be left as 'orphaned')
    assert!(collision.num_points() == 12 || collision.num_points() == 13);

    // If there are 13 points, one should have no edges any more (as another should have been chosen as the shared point)
    if collision.num_points() == 13 {
        let mut num_orphaned_points = 0;
        for point_idx in 0..13 {
            let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();
            if edges.is_empty() {
                num_orphaned_points += 1;
            }
        }

        assert!(num_orphaned_points <= 1);
    }

    // Check the intersection points
    let mut num_intersects = 0;
    for point_idx in 0..collision.num_points() {
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();

        if edges.len() == 2 {
            num_intersects += 1;

            if edges[0].start_point().distance_to(&Coord2(2.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 0.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(1.0, 1.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(2.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(5.0, 5.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 6.0)) < 0.1));
            } else {
                // These are the only four intersection points that should exist
                println!("{:?}", edges[0].start_point());
                assert!(false)
            }
        } else if edges.len() == 1 {
            let edge = edges.iter().nth(0).unwrap();
            let start_point = edge.start_point();

            assert!(
                (start_point.x() - 1.0).abs() < 0.01
                    || (start_point.x() - 5.0).abs() < 0.01
                    || (start_point.x() - 2.0).abs() < 0.01
                    || (start_point.x() - 4.0).abs() < 0.01
            );
            assert!(
                (start_point.y() - 1.0).abs() < 0.01
                    || (start_point.y() - 5.0).abs() < 0.01
                    || (start_point.y() - 0.0).abs() < 0.01
                    || (start_point.y() - 6.0).abs() < 0.01
            );
        } else {
            // Should only be 1 edge (corners) or 2 edges (collision points)
            println!("{:?}", edges);
            assert!(edges.len() <= 2);
        }
    }

    assert!(num_intersects == 4);
}

#[test]
fn collision_exactly_on_edge_src() {
    // Two rectangles, with the collision point already subdivided
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(2.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 0.0))
        .line_to(Coord2(4.0, 6.0))
        .line_to(Coord2(2.0, 6.0))
        .line_to(Coord2(2.0, 0.0))
        .line_to(Coord2(4.0, 0.0))
        .build();

    let rectangle1 = GraphPath::from_path(&rectangle1, ());
    let rectangle2 = GraphPath::from_path(&rectangle2, ());

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.05);

    // 12 points in the collision (but we can allow for the shared point to be left as 'orphaned')
    assert!(collision.num_points() == 12 || collision.num_points() == 13);

    // If there are 13 points, one should have no edges any more (as another should have been chosen as the shared point)
    if collision.num_points() == 13 {
        let mut num_orphaned_points = 0;
        for point_idx in 0..13 {
            let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();
            if edges.is_empty() {
                num_orphaned_points += 1;
            }
        }

        assert!(num_orphaned_points <= 1);
    }

    // Check the intersection points
    let mut num_intersects = 0;
    for point_idx in 0..collision.num_points() {
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();

        if edges.len() == 2 {
            num_intersects += 1;

            if edges[0].start_point().distance_to(&Coord2(2.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 0.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(1.0, 1.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(2.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(5.0, 5.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 6.0)) < 0.1));
            } else {
                // These are the only four intersection points that should exist
                println!("{:?}", edges[0].start_point());
                assert!(false)
            }
        } else if edges.len() == 1 {
            let edge = edges.iter().nth(0).unwrap();
            let start_point = edge.start_point();

            assert!(
                (start_point.x() - 1.0).abs() < 0.01
                    || (start_point.x() - 5.0).abs() < 0.01
                    || (start_point.x() - 2.0).abs() < 0.01
                    || (start_point.x() - 4.0).abs() < 0.01
            );
            assert!(
                (start_point.y() - 1.0).abs() < 0.01
                    || (start_point.y() - 5.0).abs() < 0.01
                    || (start_point.y() - 0.0).abs() < 0.01
                    || (start_point.y() - 6.0).abs() < 0.01
            );
        } else {
            // Should only be 1 edge (corners) or 2 edges (collision points)
            println!("{:?}", edges);
            assert!(edges.len() <= 2);
        }
    }

    assert!(num_intersects == 4);
}

#[test]
fn collision_exactly_on_edge_tgt() {
    // Two rectangles, with the collision point already subdivided
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 0.0))
        .line_to(Coord2(4.0, 6.0))
        .line_to(Coord2(2.0, 6.0))
        .line_to(Coord2(2.0, 1.0))
        .line_to(Coord2(2.0, 0.0))
        .line_to(Coord2(4.0, 0.0))
        .build();

    let rectangle1 = GraphPath::from_path(&rectangle1, ());
    let rectangle2 = GraphPath::from_path(&rectangle2, ());

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.02);

    // 12 points in the collision (but we can allow for the shared point to be left as 'orphaned')
    assert!(collision.num_points() == 12 || collision.num_points() == 13);

    // If there are 13 points, one should have no edges any more (as another should have been chosen as the shared point)
    if collision.num_points() == 13 {
        let mut num_orphaned_points = 0;
        for point_idx in 0..13 {
            let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();
            if edges.is_empty() {
                num_orphaned_points += 1;
            }
        }

        assert!(num_orphaned_points <= 1);
    }

    // Check the intersection points
    let mut num_intersects = 0;
    for point_idx in 0..collision.num_points() {
        let edges = collision.edges_for_point(point_idx).collect::<Vec<_>>();

        if edges.len() == 2 {
            num_intersects += 1;

            if edges[0].start_point().distance_to(&Coord2(2.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 0.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(1.0, 1.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 1.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(2.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(2.0, 1.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 5.0)) < 0.1));
            } else if edges[0].start_point().distance_to(&Coord2(4.0, 5.0)) < 0.1 {
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(5.0, 5.0)) < 0.1));
                assert!(edges
                    .iter()
                    .any(|edge| edge.end_point().distance_to(&Coord2(4.0, 6.0)) < 0.1));
            } else {
                // These are the only four intersection points that should exist
                println!("{:?}", edges[0].start_point());
                assert!(false)
            }
        } else if edges.len() == 1 {
            let edge = edges.iter().nth(0).unwrap();
            let start_point = edge.start_point();

            assert!(
                (start_point.x() - 1.0).abs() < 0.01
                    || (start_point.x() - 5.0).abs() < 0.01
                    || (start_point.x() - 2.0).abs() < 0.01
                    || (start_point.x() - 4.0).abs() < 0.01
            );
            assert!(
                (start_point.y() - 1.0).abs() < 0.01
                    || (start_point.y() - 5.0).abs() < 0.01
                    || (start_point.y() - 0.0).abs() < 0.01
                    || (start_point.y() - 6.0).abs() < 0.01
            );
        } else {
            // Should only be 1 edge (corners) or 2 edges (collision points)
            println!("{:?}", edges);
            assert!(edges.len() <= 2);
        }
    }

    assert!(num_intersects == 4);
}

fn to_collision_with_edges<'a, Point, Label>(
    collisions: Vec<(GraphRayCollision, f64, f64, Coord2)>,
    graph_path: &'a GraphPath<Point, Label>,
) -> Vec<(GraphEdge<'a, Point, Label>, f64, f64)>
where
    Point: Coordinate + Coordinate2D,
    Label: Copy,
{
    collisions
        .into_iter()
        .map(move |(collision, curve_t, line_t, _pos)| {
            let edge = collision.edge();
            (graph_path.get_edge(edge), curve_t, line_t)
        })
        .collect()
}

#[test]
fn cast_ray_to_rectangle_corner() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Collide against the top-left corner
    let collision = rectangle1.ray_collisions(&(Coord2(0.0, 0.0), Coord2(1.0, 1.0)));
    let collision = to_collision_with_edges(collision, &rectangle1);

    assert!(!collision.is_empty());

    let collision = &collision[0];
    assert!(collision.0.start_point() == Coord2(1.0, 1.0));
    assert!((collision.1 - 0.0).abs() < 0.01);
}

#[test]
fn casting_ray_to_exact_point_produces_one_collision() {
    // A ray hitting an exact point in the path might produce a collision on both the 'entering' and 'leaving' edge, but should pick
    // one. t=1 on the 'leaving' edge is the same as t=0 on the 'entering' one so either of these two are valid return values

    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Collide against the top-left corner
    let collision = rectangle1.ray_collisions(&(Coord2(0.0, 0.0), Coord2(1.0, 1.0)));
    let collision = to_collision_with_edges(collision, &rectangle1);

    let collisions_with_corner = collision
        .into_iter()
        .filter(|(edge, curve_t, _line_t)| {
            edge.point_at_pos(*curve_t).distance_to(&Coord2(1.0, 1.0)) < 0.1
        })
        .collect::<Vec<_>>();
    assert!(!collisions_with_corner.is_empty());
    assert!(collisions_with_corner.len() != 2);
    assert!(collisions_with_corner.len() == 1);
}

#[test]
fn casting_ray_across_corner_produces_no_collision() {
    // If a ray hits a point such that it doesn't cross into or out of the shape, it should not count as a collision
    // (For a closed path, this should ensure there are never an odd number of collisions)
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Cast a ray so that it 'grazes' the corner of the rectangle (without crossing into it)
    let collision = rectangle1.ray_collisions(&(Coord2(0.0, 2.0), Coord2(2.0, 0.0)));

    assert!(collision.len() != 1);
    assert!(collision.is_empty());
}

#[test]
fn casting_ray_to_intersection_point_produces_two_collisions() {
    // A ray hitting an exact point that is an intersection (has two edges leaving it) should produce two collisions, one on each edge
    // ... also this case where we have an overlapping line might be weird (but I don't think we'll generate it properly yet):
    //
    //   +-----+
    //   |     |
    //   |     +----+
    //   |     |    |
    //   |     +----+
    //   |     |
    //   +-----+
    //
    // (There's an intersection where there are two edges entering it but only one leaving)
    //
    // This test should still be valid if the 'shared' edge is stored in the graph as two edges

    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 2.0))
        .line_to(Coord2(4.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 2.0))
        .line_to(Coord2(4.0, 2.0))
        .build();
    let rectangle2 = GraphPath::from_path(&rectangle2, ());

    // Collide them
    let collided = rectangle1.collide(rectangle2, 0.01);

    // Collision should be at (5, 3), so aim a ray there
    let collision = collided.ray_collisions(&(Coord2(0.0, 0.0), Coord2(5.0, 3.0)));
    let collision = to_collision_with_edges(collision, &collided);

    let collisions_with_corner = collision
        .into_iter()
        .filter(|(edge, curve_t, _line_t)| {
            edge.point_at_pos(*curve_t).distance_to(&Coord2(5.0, 3.0)) < 0.1
        })
        .collect::<Vec<_>>();
    assert!(!collisions_with_corner.is_empty());
    assert!(collisions_with_corner.len() != 4);
    assert!(collisions_with_corner.len() == 2);
}

#[test]
fn cast_ray_across_rectangle() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Collide across the center of the rectangle
    let collision = rectangle1.ray_collisions(&(Coord2(0.0, 3.0), Coord2(6.0, 3.0)));
    let collision = to_collision_with_edges(collision, &rectangle1);

    assert!(!collision.is_empty());

    let collision = &collision[0];
    assert!(
        collision
            .0
            .point_at_pos(collision.1)
            .distance_to(&Coord2(1.0, 3.0))
            < 0.001
    );
    assert!(collision.0.start_point() == Coord2(1.0, 1.0));
    assert!((collision.1 - 0.5).abs() < 0.01);
}

#[test]
fn cast_ray_to_rectangle_far_corner() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Collide against all corners
    let collision = rectangle1.ray_collisions(&(Coord2(0.0, 0.0), Coord2(6.0, 6.0)));
    let collision = to_collision_with_edges(collision, &rectangle1);

    assert!(!collision.is_empty());

    let collision = &collision[0];
    assert!(collision.0.start_point() == Coord2(1.0, 1.0));
    assert!((collision.1 - 0.0).abs() < 0.01);
}

#[test]
fn cast_ray_to_rectangle_far_corner_backwards() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Collide against all corners
    let collision = rectangle1.ray_collisions(&(Coord2(6.0, 6.0), Coord2(0.0, 0.0)));
    let collision = to_collision_with_edges(collision, &rectangle1);

    assert!(!collision.is_empty());

    let collision = &collision[0];
    assert!(collision.0.start_point().distance_to(&Coord2(5.0, 5.0)) < 0.1);
    assert!((collision.1 - 0.0).abs() < 0.01);
}

#[test]
fn cast_ray_to_nowhere() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Line that entirely misses the rectangle
    let collision = rectangle1.ray_collisions(&(Coord2(0.0, 0.0), Coord2(0.0, 10.0)));

    assert!(collision.is_empty());
}

#[test]
fn set_simple_path_as_interior() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let mut rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Mark everything as an exterior path
    let first_edge_ref = rectangle1.all_edges().next().unwrap().into();
    rectangle1.set_edge_kind_connected(first_edge_ref, GraphPathEdgeKind::Interior);

    // All edges should be exterior
    for point_idx in 0..(rectangle1.num_points()) {
        let edges = rectangle1.edges_for_point(point_idx).collect::<Vec<_>>();

        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Interior);
    }
}

#[test]
fn set_collision_as_exterior() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());

    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 2.0))
        .line_to(Coord2(4.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 2.0))
        .line_to(Coord2(4.0, 2.0))
        .build();
    let rectangle2 = GraphPath::from_path(&rectangle2, ());

    let mut collided = rectangle1.collide(rectangle2, 0.01);

    // Mark everything as an exterior path
    let first_edge_ref = collided.edges_for_point(0).next().unwrap().into();
    collided.set_edge_kind_connected(first_edge_ref, GraphPathEdgeKind::Exterior);

    // Edges 0 -> 1, 1 -> <x>, <y> -> 2, 2 -> 3 and 3 -> 0 should all be exterior
    for point_idx in vec![0, 1, 2, 3].into_iter() {
        let edges = collided.edges_for_point(point_idx).collect::<Vec<_>>();

        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Exterior);

        let edges = collided
            .reverse_edges_for_point(point_idx)
            .collect::<Vec<_>>();

        assert!(edges.len() == 1);
        assert!(edges[0].kind() == GraphPathEdgeKind::Exterior);
    }

    // Everything else should be uncategorised
    for point_idx in 4..(collided.num_points()) {
        let edges = collided.edges_for_point(point_idx).collect::<Vec<_>>();

        assert!(edges
            .into_iter()
            .all(|edge| edge.end_point_index() < 4
                || edge.kind() == GraphPathEdgeKind::Uncategorised));
    }
}

#[test]
fn get_path_from_exterior_lines() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let mut rectangle1 = GraphPath::from_path(&rectangle1, ());

    // Mark everything as an exterior path
    let first_edge = rectangle1.edges_for_point(0).next().unwrap().into();
    rectangle1.set_edge_kind_connected(first_edge, GraphPathEdgeKind::Exterior);

    // Turn back into a path
    let rectangle2 = rectangle1.exterior_paths::<SimpleBezierPath>();

    println!("{:?}", rectangle2);

    assert!(rectangle2.len() == 1);
    assert!(rectangle2[0].start_point() == Coord2(1.0, 1.0));

    let points = rectangle2[0].points().collect::<Vec<_>>();
    assert!(points.len() == 4);

    assert!(points[0].2 == Coord2(1.0, 5.0));
    assert!(points[1].2 == Coord2(5.0, 5.0));
    assert!(points[2].2 == Coord2(5.0, 1.0));
    assert!(points[3].2 == Coord2(1.0, 1.0));
}

#[test]
fn get_path_from_exterior_lines_multiple_paths() {
    // Create a rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(11.0, 1.0))
        .line_to(Coord2(11.0, 5.0))
        .line_to(Coord2(15.0, 5.0))
        .line_to(Coord2(15.0, 1.0))
        .line_to(Coord2(11.0, 1.0))
        .build();
    let rectangle1 = GraphPath::from_path(&rectangle1, ());
    let rectangle2 = GraphPath::from_path(&rectangle2, ());
    let mut rectangle1 = rectangle1.merge(rectangle2);

    // Mark everything as an exterior path
    let first_edge = rectangle1.edges_for_point(0).next().unwrap().into();
    rectangle1.set_edge_kind_connected(first_edge, GraphPathEdgeKind::Exterior);

    let first_edge = rectangle1.edges_for_point(4).next().unwrap().into();
    rectangle1.set_edge_kind_connected(first_edge, GraphPathEdgeKind::Exterior);

    // Turn back into a path
    let rectangle3 = rectangle1.exterior_paths::<SimpleBezierPath>();

    println!("{:?}", rectangle3);

    assert!(rectangle3.len() == 2);
    assert!(rectangle3[0].start_point() == Coord2(1.0, 1.0));
    assert!(rectangle3[1].start_point() == Coord2(11.0, 1.0));

    let points = rectangle3[0].points().collect::<Vec<_>>();
    assert!(points.len() == 4);

    assert!(points[0].2 == Coord2(1.0, 5.0));
    assert!(points[1].2 == Coord2(5.0, 5.0));
    assert!(points[2].2 == Coord2(5.0, 1.0));
    assert!(points[3].2 == Coord2(1.0, 1.0));

    let points = rectangle3[1].points().collect::<Vec<_>>();
    assert!(points.len() == 4);

    assert!(points[0].2 == Coord2(11.0, 5.0));
    assert!(points[1].2 == Coord2(15.0, 5.0));
    assert!(points[2].2 == Coord2(15.0, 1.0));
    assert!(points[3].2 == Coord2(11.0, 1.0));
}

#[test]
fn collide_circles() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(12.9, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Create a graph path from the first one
    let graph_path = GraphPath::from_path(&circle1, 1);
    let graph_path = graph_path.collide(GraphPath::from_path(&circle2, 2), 0.01);

    // There are four points in each circle and there should be two collision points for 10 points total
    assert!(graph_path.num_points() == 10);

    // Display the graph
    for point_idx in 0..10 {
        println!("Point {:?}", point_idx);
        for edge in graph_path.edges_for_point(point_idx) {
            println!(
                "  {:?} -> {:?} ({:?})",
                edge.start_point(),
                edge.end_point(),
                edge.end_point_index()
            );
        }
    }

    // First four points should correspond to the four points in circle1 (and should all have one edge)
    // Some implementation details depended on here:
    //   * we preserve at least the points from the first path when colliding
    assert!(graph_path.edges_for_point(0).collect::<Vec<_>>().len() == 1);
    assert!(graph_path.edges_for_point(1).collect::<Vec<_>>().len() == 1);
    assert!(graph_path.edges_for_point(2).collect::<Vec<_>>().len() == 1);
    assert!(graph_path.edges_for_point(3).collect::<Vec<_>>().len() == 1);

    // Point 1 should lead to the intersection point
    let to_intersection = graph_path.edges_for_point(0).next().unwrap();
    let intersection_point = to_intersection.end_point_index();

    assert!(intersection_point > 3);

    // Intersection point should lead to another intersection point
    let intersection_edges = graph_path
        .edges_for_point(intersection_point)
        .collect::<Vec<_>>();
    assert!(intersection_edges.len() == 2);

    // Should lead to one point in the second circle, and one other intersection point
    let is_intersection = |point_num| {
        graph_path
            .edges_for_point(point_num)
            .collect::<Vec<_>>()
            .len()
            > 1
    };

    assert!(intersection_edges
        .iter()
        .any(|edge| !is_intersection(edge.end_point_index())));
    assert!(intersection_edges
        .iter()
        .any(|edge| is_intersection(edge.end_point_index())));

    // The following intersection point should have one point that leads back into our path
    let following_intersection = intersection_edges
        .iter()
        .find(|edge| is_intersection(edge.end_point_index()))
        .unwrap();
    let second_intersection_edges = graph_path
        .edges_for_point(following_intersection.end_point_index())
        .collect::<Vec<_>>();

    assert!(second_intersection_edges
        .iter()
        .any(|edge| edge.end_point_index() <= 3));

    // It should also have a point that leads back to the first intersection, forming a loop
    assert!(second_intersection_edges
        .iter()
        .any(|edge| edge.end_point_index() == intersection_point));
}

#[test]
fn self_collide_simple_path() {
    let with_interior_point = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(2.0, 2.0))
        .line_to(Coord2(4.0, 2.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let mut with_interior_point = GraphPath::from_path(&with_interior_point, ());

    assert!(with_interior_point.num_points() == 6);

    // TODO: we get stuck with refining when this is set to 0.01, which should work
    with_interior_point.self_collide(0.01);

    println!("{:?}", with_interior_point.num_points());
    println!("{:?}", with_interior_point);

    // Should be a single collision (so one extra point)
    assert!(with_interior_point.num_points() == 7);

    // One intersection
    let num_intersections = (0..(with_interior_point.num_points()))
        .into_iter()
        .filter(|point_idx| with_interior_point.edges_for_point(*point_idx).count() > 1)
        .count();
    assert!(num_intersections == 1);
}

#[test]
fn collide_at_shared_point() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 5.0)) // Shared point
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Add them together
    let graph = GraphPath::from_path(&rectangle1, ());
    let graph = graph.collide(GraphPath::from_path(&rectangle2, ()), 0.01);

    // Should be two points at 3.0, 5.0 with only one having any edges
    let edges_at_shared = graph
        .all_edges()
        .filter(|edge| edge.start_point().distance_to(&Coord2(3.0, 5.0)) < 0.1)
        .collect::<Vec<_>>();

    assert!(edges_at_shared.len() == 2);
    assert!(edges_at_shared[0].start_point_index() == edges_at_shared[1].start_point_index());
    assert!(
        edges_at_shared[0]
            .end_point()
            .distance_to(&Coord2(1.0, 5.0))
            < 0.1
    );
    assert!(
        edges_at_shared[1]
            .end_point()
            .distance_to(&Coord2(3.0, 3.0))
            < 0.1
    );

    let points_at_shared = (0..(graph.num_points()))
        .into_iter()
        .filter(|point_idx| {
            graph
                .point_position(*point_idx)
                .distance_to(&Coord2(3.0, 5.0))
                < 0.01
        })
        .collect::<Vec<_>>();
    assert!(points_at_shared.len() == 2);
}

#[test]
pub fn collide_rectangle_with_self() {
    // Create the two rectangles
    let rectangle = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let rectangle1 = GraphPath::from_path(&rectangle, 1);
    let rectangle2 = GraphPath::from_path(&rectangle, 2);

    // Collide them
    let collision = rectangle1.collide(rectangle2, 0.1);

    println!("{:?}", collision);

    // 8 points in the collision (4 'orphaned' with no edges)
    assert!(collision.num_points() == 8);

    let mut num_connected_points = 0;
    for point_idx in 0..8 {
        let num_edges = collision.edges_for_point(point_idx).count();

        assert!(num_edges == 2 || num_edges == 0);
        if num_edges != 0 {
            num_connected_points += 1
        }
    }

    assert!(num_connected_points == 4);
}

#[test]
fn ray_collide_along_convex_edge() {
    // Just one rectangle
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    // Collide along the vertical seam of this graph
    let gp = GraphPath::from_path(&rectangle1, PathLabel(0, PathDirection::Clockwise));

    let collisions_seam = gp.ray_collisions(&(Coord2(5.0, 0.0), Coord2(5.0, 5.0)));
    let collisions_no_seam = gp.ray_collisions(&(Coord2(4.9, 0.0), Coord2(4.9, 5.0)));

    assert!(collisions_no_seam.len() == 2);

    // As the ray never actually enters the shape along the seam, there should be 0 collisions
    println!("{:?}", collisions_seam);
    assert!(collisions_seam.len() != 2);
    assert!(collisions_seam.is_empty());
}

#[test]
fn ray_collide_along_concave_edge() {
    let concave_shape = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(6.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    // Collide along the vertical seam of this graph
    let gp = GraphPath::from_path(&concave_shape, PathLabel(0, PathDirection::Clockwise));

    let collisions_seam = gp.ray_collisions(&(Coord2(5.0, 0.0), Coord2(5.0, 5.0)));
    let collisions_no_seam = gp.ray_collisions(&(Coord2(4.9, 0.0), Coord2(4.9, 5.0)));

    assert!(collisions_no_seam.len() == 2);

    // The shape is concave and the ray should enter it
    println!("{:?}", collisions_seam);
    assert!(collisions_seam.len() != 1);
    assert!(collisions_seam.len() == 2);
}

#[test]
fn ray_collide_along_seam_with_intersection() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(3.0, 3.0))
        .line_to(Coord2(7.0, 3.0))
        .line_to(Coord2(7.0, 7.0))
        .line_to(Coord2(3.0, 7.0))
        .line_to(Coord2(3.0, 3.0))
        .build();

    // Collide along the vertical seam of this graph
    let gp = GraphPath::from_path(&rectangle1, PathLabel(0, PathDirection::Clockwise)).collide(
        GraphPath::from_path(&rectangle2, PathLabel(1, PathDirection::Clockwise)),
        0.01,
    );

    println!("{:?}", gp);

    let collisions_seam = gp.ray_collisions(&(Coord2(5.0, 0.0), Coord2(5.0, 5.0)));
    let collisions_no_seam = gp.ray_collisions(&(Coord2(5.1, 0.0), Coord2(5.1, 5.0)));

    // Should collide with the line crossing the intersection, and the top line (so two collisions total)
    assert!(collisions_no_seam.len() == 2);
    assert!(collisions_seam.len() != 5);
    assert!(collisions_seam.len() != 4);
    assert!(collisions_seam.len() != 3);
    assert!(collisions_seam.len() != 1);
    assert!(collisions_seam.len() & 1 == 0);
    assert!(collisions_seam.len() == 2);

    let ray = (Coord2(5.0, 0.0), Coord2(5.0, 5.0));
    let first_collision = ray.point_at_pos(collisions_seam[0].2);
    let second_collision = ray.point_at_pos(collisions_seam[1].2);

    println!("{:?} {:?}", first_collision, second_collision);

    assert!(first_collision.distance_to(&Coord2(5.0, 3.0)) < 0.1);
    assert!(second_collision.distance_to(&Coord2(5.0, 7.0)) < 0.1);
}

#[test]
fn ray_collide_seam_with_intersection() {
    // Two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(4.0, 4.0))
        .line_to(Coord2(6.0, 4.0))
        .line_to(Coord2(6.0, 6.0))
        .line_to(Coord2(4.0, 6.0))
        .line_to(Coord2(4.0, 4.0))
        .build();

    let graph_path = GraphPath::from_path(&rectangle1, 1);
    let graph_path = graph_path.collide(GraphPath::from_path(&rectangle2, 2), 0.01);

    let collisions = graph_path.ray_collisions(&(Coord2(5.0, 6.0), Coord2(5.0, 5.0)));

    assert!(collisions.len() == 2);
    assert!(!collisions[0].0.is_intersection());
    assert!(collisions[1].0.is_intersection());
}

#[test]
fn ray_collide_with_edges_and_convex_point_intersection() {
    let with_interior_point = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(2.0, 2.0))
        .line_to(Coord2(4.0, 2.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut with_interior_point = GraphPath::from_path(&with_interior_point, ());
    with_interior_point.self_collide(0.01);

    let collisions = with_interior_point.ray_collisions(&(Coord2(0.0, 3.0), Coord2(1.0, 3.0)));

    println!("{:?}", with_interior_point);
    println!("{:?}", collisions);

    assert!(collisions.len() == 4);
    assert!(collisions[1].0.is_intersection());
    assert!(collisions[2].0.is_intersection());
}

#[test]
fn ray_collide_doughnuts_near_intersection() {
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle1 = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(9.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle2 = Circle::new(Coord2(9.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    let mut circle1 = GraphPath::from_path(&circle1, ());
    circle1 = circle1.merge(GraphPath::from_path(&inner_circle1, ()));
    let mut circle2 = GraphPath::from_path(&circle2, ());
    circle2 = circle2.merge(GraphPath::from_path(&inner_circle2, ()));

    let graph_path = circle1.collide(circle2, 0.1);

    let collisions = graph_path.ray_collisions(&(
        Coord2(7.000584357101389, 8.342524209216537),
        Coord2(6.941479643691172, 8.441210096108172),
    ));
    let collision_count = collisions.len();

    println!("{:?}", collisions);

    assert!((collision_count & 1) == 0);
}

#[test]
fn ray_collide_doughnuts_many_angles() {
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle1 = Circle::new(Coord2(5.0, 5.0), 3.9).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(9.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let inner_circle2 = Circle::new(Coord2(9.0, 5.0), 3.9).to_path::<SimpleBezierPath>();

    let mut circle1 = GraphPath::from_path(&circle1, ());
    circle1 = circle1.merge(GraphPath::from_path(&inner_circle1, ()));
    let mut circle2 = GraphPath::from_path(&circle2, ());
    circle2 = circle2.merge(GraphPath::from_path(&inner_circle2, ()));

    let graph_path = circle1.collide(circle2, 0.01);

    for angle in 0..3600 {
        let angle = (angle as f64) / 3600.0;
        let angle = (angle / 360.0) * 2.0 * f64::consts::PI;
        let ray_start = Coord2(9.0, 5.0) + Coord2(5.0 * angle.sin(), 5.0 * angle.cos());
        let ray_end = Coord2(5.0, 5.0) - Coord2(5.0 * angle.sin(), 5.0 * angle.cos());

        let collisions = graph_path.ray_collisions(&(ray_start, ray_end));
        let collision_count = collisions.len();

        assert!((collision_count & 1) == 0);
    }
}

#[test]
fn self_collide_removes_shared_point_1() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path = GraphPath::from_path(&path, ());
    graph_path.self_collide(0.01);

    let mut edges_ending_at_center = vec![];
    for edge in graph_path.all_edges() {
        if edge.end_point().distance_to(&Coord2(3.0, 3.0)) < 0.01 {
            edges_ending_at_center.push(edge);
        }
    }

    assert!(edges_ending_at_center.len() == 2);
    assert!(edges_ending_at_center
        .iter()
        .all(|edge| edge.end_point_index() == edges_ending_at_center[0].end_point_index()));
}

#[test]
fn self_collide_removes_shared_point_2() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(3.0, 3.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path = GraphPath::from_path(&path, ());
    graph_path.self_collide(0.01);

    let mut edges_ending_at_center = vec![];
    for edge in graph_path.all_edges() {
        if edge.end_point().distance_to(&Coord2(3.0, 3.0)) < 0.01 {
            edges_ending_at_center.push(edge);
        }
    }

    assert!(edges_ending_at_center.len() == 2);
    assert!(edges_ending_at_center
        .iter()
        .all(|edge| edge.end_point_index() == edges_ending_at_center[0].end_point_index()));
}

#[test]
fn self_collide_divides_lines_1() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(3.0, 0.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path = GraphPath::from_path(&path, ());

    assert!(graph_path.all_edges().count() == 5);

    graph_path.self_collide(0.01);

    assert!(graph_path.all_edges().count() == 9);
}

#[test]
fn self_collide_divides_lines_2() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(3.0, 0.985))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path = GraphPath::from_path(&path, ());

    assert!(graph_path.all_edges().count() == 5);

    graph_path.self_collide(0.01);

    println!("{:?}", graph_path);
    assert!(graph_path.all_edges().count() == 9);
}

#[test]
fn self_collide_divides_lines_3() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(3.0, 0.999))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path = GraphPath::from_path(&path, ());

    assert!(graph_path.all_edges().count() == 5);

    graph_path.self_collide(0.01);

    println!("{:?}", graph_path);

    // So close we should just collide as if the 3.0, 0.999 point is at 3.0, 1.0
    assert!(!graph_path
        .all_edges()
        .any(|edge| edge.start_point().distance_to(&edge.end_point()) < 0.001));
    assert!(graph_path.all_edges().count() != 9); // Technically valid, indicates a change in the precision of the collision
    assert!(graph_path.all_edges().count() != 7);
    assert!(graph_path.all_edges().count() == 6);
}

#[test]
fn heal_one_line_gap() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path = GraphPath::from_path(&path, ());
    let edges = (0..4)
        .into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).next().unwrap().into())
        .collect::<Vec<_>>();

    graph_path.set_edge_kind(edges[0], GraphPathEdgeKind::Exterior);
    graph_path.set_edge_kind(edges[2], GraphPathEdgeKind::Exterior);
    graph_path.set_edge_kind(edges[3], GraphPathEdgeKind::Exterior);

    assert!(graph_path.get_edge(edges[1]).kind() == GraphPathEdgeKind::Uncategorised);

    assert!(graph_path.heal_exterior_gaps());

    assert!(graph_path.get_edge(edges[1]).kind() == GraphPathEdgeKind::Exterior);
}

#[test]
fn heal_two_line_gap() {
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();

    let mut graph_path = GraphPath::from_path(&path, ());
    let edges = (0..4)
        .into_iter()
        .map(|point_idx| graph_path.edges_for_point(point_idx).next().unwrap().into())
        .collect::<Vec<_>>();

    graph_path.set_edge_kind(edges[0], GraphPathEdgeKind::Exterior);
    graph_path.set_edge_kind(edges[3], GraphPathEdgeKind::Exterior);

    assert!(graph_path.get_edge(edges[1]).kind() == GraphPathEdgeKind::Uncategorised);
    assert!(graph_path.get_edge(edges[2]).kind() == GraphPathEdgeKind::Uncategorised);

    assert!(graph_path.heal_exterior_gaps());

    assert!(graph_path.get_edge(edges[1]).kind() == GraphPathEdgeKind::Exterior);
    assert!(graph_path.get_edge(edges[2]).kind() == GraphPathEdgeKind::Exterior);
}

#[test]
fn ray_cast_at_tiny_line_1() {
    // Line with two points .011 apart (just above CLOSE_DISTANCE)
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(2.9945, 5.0))
        .line_to(Coord2(3.0055, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = GraphPath::from_path(&path, ());

    // Should be able to cast a ray and hit our line and none of the others
    let collisions = path.ray_collisions(&(Coord2(3.0, 0.0), Coord2(3.0, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);

    // Tiny line is points 2,3
    let small_edge = path.get_edge(collisions[1].0.edge());
    assert!(small_edge.start_point_index() == 2);
    assert!(small_edge.end_point_index() == 3);

    // Not an intersection
    assert!(!collisions[1].0.is_intersection());
}

#[test]
fn ray_cast_at_tiny_line_2() {
    // Line with several points .011 apart (just above CLOSE_DISTANCE)
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(2.9835, 5.0))
        .line_to(Coord2(2.9945, 5.0))
        .line_to(Coord2(3.0055, 5.0))
        .line_to(Coord2(3.0165, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = GraphPath::from_path(&path, ());

    // Should be able to cast a ray and hit our line and none of the others
    for p in 0..10 {
        let offset = ((p as f64) / 10.0) * 0.01;

        let collisions =
            path.ray_collisions(&(Coord2(2.9945 + offset, 0.0), Coord2(2.9945 + offset, 1.0)));
        println!("{:?}", collisions);
        assert!(collisions.len() == 2);

        // Tiny line is points 3,4
        let small_edge = path.get_edge(collisions[1].0.edge());
        assert!(small_edge.start_point_index() == 3);
        assert!(small_edge.end_point_index() == 4);

        // Not an intersection
        assert!(!collisions[1].0.is_intersection());
    }
}

#[test]
fn ray_cast_at_tiny_line_3() {
    // Line with several points .011 apart (just above CLOSE_DISTANCE)
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(2.9835, 5.0))
        .line_to(Coord2(2.9945, 5.0))
        .line_to(Coord2(3.0055, 5.0))
        .line_to(Coord2(3.0165, 5.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = GraphPath::from_path(&path, ());

    // Aim at the end of the line
    let collisions = path.ray_collisions(&(Coord2(3.0055, 0.0), Coord2(3.0055, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);

    // Tiny line is points 4,5
    let small_edge = path.get_edge(collisions[1].0.edge());
    assert!(small_edge.start_point_index() == 4);
    assert!(small_edge.end_point_index() == 5);

    // Not an intersection
    assert!(!collisions[1].0.is_intersection());
}

#[test]
fn ray_cast_at_tiny_line_4() {
    // Line with a tiny zig-zag
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(3.0055, 5.0))
        .line_to(Coord2(2.9945, 4.99))
        .line_to(Coord2(3.0055, 4.99))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = GraphPath::from_path(&path, ());

    // Should be able to cast a ray and hit our line and none of the others
    let collisions = path.ray_collisions(&(Coord2(3.0, 0.0), Coord2(3.0, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 4);

    // Tiny line is points 3,4
    let small_edge = path.get_edge(collisions[1].0.edge());
    assert!(small_edge.start_point_index() == 3);
    assert!(small_edge.end_point_index() == 4);

    // Not an intersection
    assert!(!collisions[1].0.is_intersection());
}

#[test]
fn ray_cast_at_tiny_line_5() {
    // Very long line followed by very short line
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(10000.0, 5.0))
        .line_to(Coord2(10000.02, 5.0))
        .line_to(Coord2(20000.0, 5.0))
        .line_to(Coord2(20000.0, 1.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = GraphPath::from_path(&path, ());

    // Should be able to cast a ray and hit our line and none of the others
    let collisions = path.ray_collisions(&(Coord2(9999.99, 0.0), Coord2(9999.99, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);

    let collisions = path.ray_collisions(&(Coord2(9999.999, 0.0), Coord2(9999.999, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);

    let collisions = path.ray_collisions(&(Coord2(10000.001, 0.0), Coord2(10000.001, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);

    let collisions = path.ray_collisions(&(Coord2(10000.0195, 0.0), Coord2(10000.0195, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);

    let collisions = path.ray_collisions(&(Coord2(10000.01999, 0.0), Coord2(10000.01999, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);

    let collisions = path.ray_collisions(&(Coord2(10000.0201, 0.0), Coord2(10000.0201, 1.0)));
    println!("{:?}", collisions);
    assert!(collisions.len() == 2);
}

#[test]
fn ray_cast_at_tiny_line_6() {
    // Path with a pair of lines with a known failure on them
    let path = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(525.2388916015625, 931.7135009765625))
        .curve_to(
            (
                Coord2(525.4012451171875, 931.7196044921875),
                Coord2(525.5686645507813, 931.7201538085938),
            ),
            Coord2(525.7626342773438, 931.6915893554688),
        )
        .curve_to(
            (
                Coord2(526.2460327148438, 931.761962890625),
                Coord2(526.3161010742188, 931.8532104492188),
            ),
            Coord2(526.6378173828125, 931.9375610351563),
        )
        .curve_to(
            (
                Coord2(529.997314453125, 935.0886840820313),
                Coord2(508.8724365234375, 903.5847778320313),
            ),
            Coord2(508.7933654785156, 901.745849609375),
        )
        .line_to(Coord2(700.0, 900.0))
        .line_to(Coord2(1.0, 900.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let path = GraphPath::from_path(&path, ());

    let collisions = path.ray_collisions(&(
        Coord2(543.606689453125, 925.3496704101563),
        Coord2(553.524658203125, 921.505126953125),
    ));
    println!("{:?}", collisions);
    assert!((collisions.len() & 1) == 0);
    assert!(collisions.len() == 4);
}

#[test]
fn ray_cast_grazing_circle_produces_0_hits() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(20.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Collide them into a graphpath
    let path = GraphPath::from_path(&circle1, ());
    let path = path.collide(GraphPath::from_path(&circle2, ()), 0.01);

    // Ray cast upwards, grazing the first circle
    let collisions = path.ray_collisions(&(Coord2(24.0, 0.0), Coord2(24.0, 1.0)));

    // Should not actually hit the circle
    assert!(collisions.len() != 2); // 2 collisions would produce no bug
    assert!(collisions.len() != 1);
    assert!(collisions.is_empty());
}

#[test]
fn ray_cast_close_to_circle_produces_2_hits() {
    // Two overlapping circles
    let circle1 = Circle::new(Coord2(5.0, 5.0), 4.0).to_path::<SimpleBezierPath>();
    let circle2 = Circle::new(Coord2(20.0, 5.0), 4.0).to_path::<SimpleBezierPath>();

    // Collide them into a graphpath
    let path = GraphPath::from_path(&circle1, ());
    let path = path.collide(GraphPath::from_path(&circle2, ()), 0.01);

    // Ray cast upwards, grazing the first circle
    let collisions = path.ray_collisions(&(Coord2(23.99, 0.0), Coord2(23.99, 1.0)));

    // Should not actually hit the circle
    assert!(!collisions.is_empty());
    assert!(collisions.len() != 1);
    assert!(collisions.len() == 2);
}

#[test]
pub fn ray_cast_identical_rectangles() {
    // Create the two rectangles
    let rectangle1 = BezierPathBuilder::<SimpleBezierPath>::start(Coord2(1.0, 1.0))
        .line_to(Coord2(5.0, 1.0))
        .line_to(Coord2(5.0, 5.0))
        .line_to(Coord2(1.0, 5.0))
        .line_to(Coord2(1.0, 1.0))
        .build();
    let rectangle2 = rectangle1.clone();

    let rectangle1 = GraphPath::from_path(&rectangle1, 1);
    let rectangle2 = GraphPath::from_path(&rectangle2, 2);

    // Collide them
    let path = rectangle1.collide(rectangle2, 0.1);

    // The edges are identical, so we need to process them in a consistent order
    let collisions = path.ray_collisions(&(Coord2(3.0, 0.0), Coord2(3.0, 10.0)));

    // Collides with two edges twice, so four total collisions
    assert!(collisions.len() == 4);

    // First two collisions should hit path 1 and path 2
    let edge1 = collisions[0].0.edge();
    let edge2 = collisions[1].0.edge();
    let edge3 = collisions[2].0.edge();
    let edge4 = collisions[3].0.edge();

    let edge1 = path.get_edge(edge1);
    let edge2 = path.get_edge(edge2);
    let edge3 = path.get_edge(edge3);
    let edge4 = path.get_edge(edge4);

    // edge1, edge2 and edge3, edge4 should all have the same start and end points (ie, be duplicate edges)
    assert!(edge1.start_point_index() == edge2.start_point_index());
    assert!(edge1.end_point_index() == edge2.end_point_index());
    assert!(edge3.start_point_index() == edge4.start_point_index());
    assert!(edge3.end_point_index() == edge4.end_point_index());

    assert!(edge1.start_point_index() != edge3.start_point_index());

    // The collisions must be for the two different paths
    assert!(edge1.label() != edge2.label());
    assert!(edge3.label() != edge4.label());

    // The entry and exit collisions must be in a consistent order (ie 1 -> 2 -> 2 -> 1 or 2 -> 1 -> 1 -> 2)
    // (This is so that one path is on the outside and one path is on the inside consistently, and is only necessary
    // when the edges precisely overlap)
    assert!(edge1.label() != edge3.label());
}
