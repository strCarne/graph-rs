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
