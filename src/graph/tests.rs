use super::*;
use pretty_assertions::assert_eq;

macro_rules! assert_option_vertices {
    ($v_1:expr, $v_2:expr) => {
        assert_eq!($v_1.is_none(), $v_2.is_none());
        if let Some(result) = $v_1 {
            let expected = $v_2.unwrap();
            assert_eq!(result.key(), expected.key());
            assert_eq!(result.value, expected.value);

            for (result_edge, expected_edge) in result
                .adjancency_list()
                .iter()
                .zip(expected.adjancency_list().iter())
            {
                assert!(result_edge == expected_edge);
            }
        }
    };
}

#[test]
fn insert_vertex_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    let insertions = vec![
        Vertex::new(1, 1),
        Vertex::new(2, 2),
        Vertex::new(1, 3),
        Vertex::new(2, 1),
    ]
    .into_iter();

    let expected_results =
        vec![None, None, Some(Vertex::new(1, 1)), Some(Vertex::new(2, 2))].into_iter();

    assert_eq!(
        insertions.len(),
        expected_results.len(),
        "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
    );

    for (insertion, expected) in insertions.zip(expected_results) {
        let result = graph.insert_vertex(insertion);
        assert_option_vertices!(result, expected);
    }
}

#[test]
fn get_vertex_test() {
    let mut graph: Graph<i32, i32> = Graph::new();
    graph.insert(1, 1);
    graph.insert(2, 2);

    let keys = vec![1, 2, 3].into_iter();

    let holder = vec![Vertex::new(1, 1), Vertex::new(2, 2)];
    let expected_results = vec![Some(&holder[0]), Some(&holder[1]), None].into_iter();

    assert_eq!(
        keys.len(),
        expected_results.len(),
        "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
    );

    for (key, expected) in keys.zip(expected_results) {
        let result = graph.get_vertex(&key);
        assert_option_vertices!(result, expected);
    }
}

#[test]
#[ignore = "same as get_vertex_test, but returns a mutable reference"]
fn get_vertex_mut_test() {
    unimplemented!()
}

#[test]
fn remove_vertex_test() {
    let mut graph: Graph<i32, i32> = Graph::new();
    graph.insert(1, 1);
    graph.insert(2, 2);

    let keys = vec![1, 2, 1].into_iter();
    let expected_results = vec![Some(Vertex::new(1, 1)), Some(Vertex::new(2, 2)), None].into_iter();

    assert_eq!(
        keys.len(),
        expected_results.len(),
        "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
    );

    for (key, expected) in keys.zip(expected_results) {
        let result = graph.remove_vertex(&key);
        assert_option_vertices!(result, expected);
    }
}

#[test]
#[ignore = "based on Graph::insert_vertex, which is already tested"]
fn insert_test() {
    unimplemented!()
}

#[test]
#[ignore = "based on Graph::get_vertex, which is already tested"]
fn get_test() {
    unimplemented!()
}

#[test]
#[ignore = "based on Graph::get_mut_vertex, which is already tested"]
fn get_mut_test() {
    unimplemented!()
}

#[test]
#[ignore = "based on Graph::remove_vertex, which is already tested"]
fn remove_test() {
    unimplemented!()
}

#[test]
fn dfs_empty_test() {
    let graph: Graph<i32, i32> = Graph::new();

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.dfs(&0) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed.len(), 0);
}

#[test]
fn dfs_simple_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    graph.insert(1, 1);
    graph.insert(2, 2);
    graph.insert(3, 3);
    graph.insert(4, 4);

    graph
        .insert_edge_unweighted(1, 2)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(1, 3)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(3, 4)
        .expect("must contain both vertices");

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.dfs(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 3, 4, 2]);
}

#[test]
fn dfs_hard_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    let vertecies = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
        (10, 10),
        (11, 11),
        (12, 12),
        (13, 13),
        (14, 14),
        (15, 15),
        (16, 16),
    ]
    .into_iter();

    for (key, value) in vertecies {
        graph.insert(key, value);
    }

    let edges = vec![
        (1, 2),
        (2, 3),
        (2, 10),
        (3, 4),
        (3, 11),
        (4, 5),
        (5, 6),
        (5, 6),
        (6, 7),
        (7, 5),
        (8, 9),
        (9, 1),
        (10, 6),
        (12, 4),
        (12, 14),
        (13, 12),
        (15, 16),
        (16, 14),
    ]
    .into_iter();

    for (from, to) in edges {
        graph
            .insert_edge_unweighted(from, to)
            .expect("must contain both vertices");
    }

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.dfs(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 2, 10, 6, 7, 5, 3, 11, 4,]);
}

#[test]
fn dfs_mut_empty_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.dfs_mut(&0) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed.len(), 0);
}

#[test]
fn dfs_mut_simple_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    graph.insert(1, 1);
    graph.insert(2, 2);
    graph.insert(3, 3);
    graph.insert(4, 4);

    graph
        .insert_edge_unweighted(1, 2)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(1, 3)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(3, 4)
        .expect("must contain both vertices");

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.dfs_mut(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 3, 4, 2]);
}

#[test]
fn dfs_mut_hard_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    let vertecies = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
        (10, 10),
        (11, 11),
        (12, 12),
        (13, 13),
        (14, 14),
        (15, 15),
        (16, 16),
    ]
    .into_iter();

    for (key, value) in vertecies {
        graph.insert(key, value);
    }

    let edges = vec![
        (1, 2),
        (2, 3),
        (2, 10),
        (3, 4),
        (3, 11),
        (4, 5),
        (5, 6),
        (5, 6),
        (6, 7),
        (7, 5),
        (8, 9),
        (9, 1),
        (10, 6),
        (12, 4),
        (12, 14),
        (13, 12),
        (15, 16),
        (16, 14),
    ]
    .into_iter();

    for (from, to) in edges {
        graph
            .insert_edge_unweighted(from, to)
            .expect("must contain both vertices");
    }

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.dfs_mut(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 2, 10, 6, 7, 5, 3, 11, 4,]);
}

#[test]
fn bfs_empty_test() {
    let graph: Graph<i32, i32> = Graph::new();

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.bfs(&0) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed.len(), 0);
}

#[test]
fn bfs_simple_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    graph.insert(1, 1);
    graph.insert(2, 2);
    graph.insert(3, 3);
    graph.insert(4, 4);

    graph
        .insert_edge_unweighted(1, 2)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(1, 3)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(3, 4)
        .expect("must contain both vertices");

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.bfs(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 2, 3, 4]);
}

#[test]
fn bfs_hard_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    let vertecies = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
        (10, 10),
        (11, 11),
        (12, 12),
        (13, 13),
        (14, 14),
        (15, 15),
        (16, 16),
    ]
    .into_iter();

    for (key, value) in vertecies {
        graph.insert(key, value);
    }

    let edges = vec![
        (1, 2),
        (2, 3),
        (2, 10),
        (3, 4),
        (3, 11),
        (4, 5),
        (5, 6),
        (5, 6),
        (6, 7),
        (7, 5),
        (8, 9),
        (9, 1),
        (10, 6),
        (12, 4),
        (12, 14),
        (13, 12),
        (15, 16),
        (16, 14),
    ]
    .into_iter();

    for (from, to) in edges {
        graph
            .insert_edge_unweighted(from, to)
            .expect("must contain both vertices");
    }

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.bfs(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 2, 3, 10, 4, 11, 6, 5, 7,]);
}

#[test]
fn bfs_mut_empty_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.bfs_mut(&0) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed.len(), 0);
}

#[test]
fn bfs_mut_simple_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    graph.insert(1, 1);
    graph.insert(2, 2);
    graph.insert(3, 3);
    graph.insert(4, 4);

    graph
        .insert_edge_unweighted(1, 2)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(1, 3)
        .expect("must contain both vertices");
    graph
        .insert_edge_unweighted(3, 4)
        .expect("must contain both vertices");

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.bfs_mut(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 2, 3, 4]);
}

#[test]
fn bfs_mut_hard_test() {
    let mut graph: Graph<i32, i32> = Graph::new();

    let vertecies = vec![
        (1, 1),
        (2, 2),
        (3, 3),
        (4, 4),
        (5, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 9),
        (10, 10),
        (11, 11),
        (12, 12),
        (13, 13),
        (14, 14),
        (15, 15),
        (16, 16),
    ]
    .into_iter();

    for (key, value) in vertecies {
        graph.insert(key, value);
    }

    let edges = vec![
        (1, 2),
        (2, 3),
        (2, 10),
        (3, 4),
        (3, 11),
        (4, 5),
        (5, 6),
        (5, 6),
        (6, 7),
        (7, 5),
        (8, 9),
        (9, 1),
        (10, 6),
        (12, 4),
        (12, 14),
        (13, 12),
        (15, 16),
        (16, 14),
    ]
    .into_iter();

    for (from, to) in edges {
        graph
            .insert_edge_unweighted(from, to)
            .expect("must contain both vertices");
    }

    let mut traversed: Vec<i32> = vec![];
    for vertex in graph.bfs_mut(&1) {
        traversed.push(vertex.key().clone());
    }

    assert_eq!(traversed, vec![1, 2, 3, 10, 4, 11, 6, 5, 7,]);
}
